

use std::clone;
use std::collections::{HashMap};
use std::hash::Hash;
use std::io::Write;
use chrono::Local;

use std::fs::{File, read_to_string};
use std::{env, io, path::{Path, PathBuf}, process::{exit, Command, ExitStatus, Stdio}}; //Path::new -> &Path plus needs Box<&Path> since it's unsized (don't implement Sized), Box or & or PathBuf(like an owned Path)  fixes it
use std::ffi::OsStr;
use tokio::time::{sleep, Duration};
use std::time::Instant;
use regex::Regex;
use std::process;

use crate::*;




/*
example of file .sconfig


prgm_name: nginx
cmd: "/usr/local/bin/nginx -c /etc/nginx/test.conf"
numprocs: 3
umask: 022
workingdir: /tmp
autostart: true
autorestart: unexpected
exitcodes: 0,2,
startretries: 3
starttime: 5
stopsignal: SIGTERM
stoptime: 10
stdout: ./test_docs/parser_tests/file.stdout
stderr: ./test_docs/parser_tests/file.stderr
env: STARTED_BY=taskmaster,ANSWER=42,\n





*/

//launch cmd 

/* 
LST PRS STATUS AND CONFIG 
FOR EVERY CMD (CMD * NB PROCESSUS COUNTS AS N CMDS )
    1.PURE CMDS   cd env  >> stdout file 
    2.WATCHERS timers  reruns   signals 
    3.CHECKERS for exit    see if exitcodes right 

! hcnage ConfigParser to ATask
*/


//were gonaj do PURE FNS   if one elem has ot be chane copy entire lst and change that one elem  OR spread data ascross mulltiple vecs 



//type ConfigParser<Task> = Vec<Task>;

//shouldi use impl instead Self::global
//static mut ALL_TASKS : Vec<Task> = vec![];
//static mut ALL_PROCESS_OF_TASKS : Vec<ProcessOfTask> = vec![];


/*fn get_command_handler_from_pid(pid: i32) -> Option<Command> {
    let process = Process::new(pid);
    let process = match process {
        Ok(process) => {
            process
        }   
        Err(e) => {
            eprintln!("Error: {:?}", e);
            return None;
        }
    }
    match process.cmdline() {
        Ok(command) => {  
            //println!("Command of PID {}: {}", pid, command);
            Some(command)
        }
        Err(err) =>  {
            eprintln!("Error: {:?}", err);
            None
        }
    }
}*/


type BuildCmd = String;

#[derive(Debug)]
pub struct ProcessOfTask<'a> {
    handler: Command,
    cmd : String,
    task_ref : &'a Task,
    pid: Option<u32>,
    final_exit_code: Option<i32>,
    nb_restarted : usize,
    is_running : bool,
    last_error : Option<String>, 
    log_file_out : String, 
    log_file_err : String, 
    dev_errors : HashMap<String, String>,


}


impl<'a> ProcessOfTask<'a>{
    pub fn new(a_task : &'a Task, log_out : String, log_err : String) -> Self {

        let build_cmd = Self::cmd_builder(a_task);

        // Create a new Command
        let mut command = Command::new("bash");

        // Change directory using `cd`
        command.arg("-c").arg(build_cmd.clone());

        // Set the command's standard output to be inherited by the parent process
        command.stdout(Stdio::inherit());

        // Set the command's standard error to be inherited by the parent process
        command.stderr(Stdio::inherit());

        Self {
            handler : command,
            cmd : build_cmd,
            task_ref : a_task,
            pid : None,
            nb_restarted : 0,
            final_exit_code: None,
            is_running : false,
            last_error : None, 
            dev_errors : HashMap::new(),
            log_file_out : log_out,
            log_file_err : log_err,

        }
    }

    fn write_dev_msg(&mut self, msg : String) {
        let now_date =  Local::now();
        self.write_to_log("out", format!("{} :: {}",now_date, msg));
    }

    fn write_dev_err(&mut self, error_msg : String) {
        let now_date = Local::now();
        self.dev_errors.insert(format!("{}",now_date), error_msg.clone());
        self.write_to_log("err", format!("{} :: {}",now_date, error_msg));
    }

    fn eprintln_dev_err(&mut self, error_msg : String) {
        let now_date =  Local::now();
        println!("{} :: {}", now_date, error_msg);
    }

    fn new_bash_cmd(cmd_and_args : String) -> Command {
        let mut command = Command::new("bash");
        command.arg("-c").arg(cmd_and_args);
        command.stdout(Stdio::inherit());
        command.stderr(Stdio::inherit());
        command
    }

    pub fn cmd_builder(a_task : &Task) -> BuildCmd {

        let working_path = a_task.workingdir.to_str();
        let stdout_path = a_task.stdout.to_str();
        let stderr_path = a_task.stderr.to_str();

        if working_path.is_none() || stderr_path.is_none() || stdout_path.is_none() {
            eprintln!("{}" , String::from("function cmd_builder conversion failed : PathBuf to_str() is None")); 
            return String::from("");
        }

        let working_path = working_path.unwrap();
        let stdout_path = stdout_path.unwrap();
        let stderr_path = stderr_path.unwrap();

        let build_cmd : String = a_task.env
            .iter()
            .map(|(key, value)| format!(" {}={} ", key, value)) // Add 10 to each value
            .collect();
        let cmd = format!("export {} && umask {} && (cd {:?} && {}) >> {:?} 2>> {:?}", build_cmd, 
                                    a_task.umask,
                                    working_path,
                                    a_task.cmd.join(" "),
                                    stdout_path,
                                    stderr_path
                                    );
        cmd
    }

    pub async fn run(&mut self) {
        /* the process is not running, once you run it, it will have a pid , and the other fields will be updated  */
        //we dont want to modify the old one so we clone   but it dont implement it ......

        //RUNS status() runs cmd
        let exit_status_res : io::Result<ExitStatus> = self.handler.status(); 
        match exit_status_res {
            Ok(exit_status) => {
                let exit_status_code_option : Option<i32> = exit_status.code();
                match exit_status_code_option {
                    Some(exit_code) => {
                        self.final_exit_code = Some(exit_code);
                        self.write_dev_msg(format!("{} [PID: {}] <{}> stopped with exitcode {}", self.task_ref.pgrm_name,self.pid.unwrap(), self.cmd, self.final_exit_code.unwrap()));
                    } 
                    None => {
                        self.pid = Some(std::process::id()); //gets current running process ... lets hope it refers this one 
                        self.is_running = true;
                        self.write_dev_msg(format!("{} [PID: {}] <{}> is now running", self.task_ref.pgrm_name, self.cmd,self.pid.unwrap()));
                    }
                }
            }
            Err(e) => self.write_dev_err(e.to_string())
        }    
    }

    async fn stop_timer_before_sigkill(&mut self, pid : u32) {
        let duration = Duration::from_secs(self.task_ref.stoptime as u64); //conversion safe as max u32 fits u64
        sleep(duration).await;
        let kill_cmd = format!("kill -9 {}", pid); //SIGKILL
        let mut handler = Self::new_bash_cmd(kill_cmd);
        let ran_cmd_res = handler.status(); //a kill fails? come on
        match ran_cmd_res {
            Ok(_) => (),
            Err(e) => self.write_dev_err(e.to_string())
        }
    }

    pub async fn stop(&mut self) {
     /* 
            write somewhere if exited gracefully
            if dont stop after $stoptime   SIGKILL
            bash kill sends SIGTERM for exit gracefully 

            
        */

        //SET ASYNC TIMER 


        //RUN STOP
        //let stop_cmd = format!("kill {}", ); //SIGTERM
        //let kill_cmd = format!("kill -9 {}", ); //SIGKILL
        match self.pid {
            Some(pid) => {
                let stop_cmd = format!("kill {}", pid); //SIGTERM
                let mut handler = Self::new_bash_cmd(stop_cmd);
                let ran_cmd_res = handler.status();
                match ran_cmd_res {
                    Ok(_) => self.stop_timer_before_sigkill(pid).await,
                    Err(e) => self.write_dev_err(e.to_string())
                }
                //write somewhere if failed to stop 
                


            },
            None => (),    
        }
        

        
        



        //DESTROY ASYNC SO IT DONT AFFECT REST OF PRGM 
    }

    
    
    fn write_to_log(&mut self, which : &str, data : String) {
        /* i can't write to log the open/write to log errors so i can only print them out on stderr */
        let log_to_write = if which == "err" { &self.log_file_err } else { &self.log_file_out };
        let mut file_opened_res = File::create(log_to_write);
        match file_opened_res {
            Ok(mut file_opened) => {
                let write_res = file_opened.write_all(data.as_bytes());
                match write_res {
                    Ok(write_res) => {
                        // Optional: Flush the buffer to ensure data is written immediately
                        let flush_res = file_opened.flush();
                        match flush_res {
                            Ok(_) => (),
                            Err(e) => self.eprintln_dev_err(e.to_string())
                        }
                    }
                    Err(e) => self.eprintln_dev_err(e.to_string())
                }
                
            }
            Err(e) => self.eprintln_dev_err(e.to_string())
        }
    }

    
    


    /* 
    pub fn get_all_ProcessOfTask(all_tasks : Vec<Task>) -> Vec<ProcessOfTask>  {
        let mut all_processes_of_tasks : Vec<ProcessOfTask> = vec![];
        for a_task in all_tasks.iter() {
            for i in 0..a_task.numprocs {
                let build_cmd = env_setup_str(a_task.clone());
                let new_ProcessOfTask = create_process_of_task(build_cmd , a_task.clone()); //OR AProcessOfTask.new(atask)
                all_processes_of_tasks.push(new_ProcessOfTask);
            }
        }
        all_processes_of_tasks
    }

    

    pub fn run_processes_of_tasks(mut all_processes_of_tasks : Vec<ProcessOfTask>) -> Vec<ProcessOfTask> {
        /* into iter drains all elems to be moved inside run_process_of_task and have it return an updated version  */
        let mut updated_processes_of_tasks : Vec<ProcessOfTask> = vec![];
        for a_process_of_task in all_processes_of_tasks.iter_mut() {
            let running_process_of_task = run_process_of_task(a_process_of_task).unwrap();
            updated_processes_of_tasks.push(running_process_of_task);
        }
        updated_processes_of_tasks
    }



    pub fn env_setup_str(a_task : Task) -> BuildCmd {

        let working_path = String::from(a_task.workingdir.to_str().unwrap());
        let stdout_path = a_task.stdout.to_str().unwrap();
        let stderr_path = a_task.stderr.to_str().unwrap();

        let build_cmd : String = a_task.env
            .iter()
            .map(|(key, value)| format!(" {}={} ", key, value)) // Add 10 to each value
            .collect();
        let cmd = format!("export {} && umask {} && (cd {:?} && {}) >> {:?} 2>> {:?}", build_cmd, 
                                    a_task.umask,
                                    working_path,
                                    a_task.cmd.join(" "),
                                    stdout_path,
                                    stderr_path
                                    );
        cmd
    }



    
    */
}
/* 
async fn watcher_time_ran() {

}

async fn async_timer_example(FN TO WATCH) {
    // Start the timer
    let start_time = Instant::now();

    //FUNC TO WATCH

    // Measure the elapsed time
    let elapsed = start_time.elapsed();

    // Print the elapsed time
    println!("Elapsed time: {:?}", elapsed);
}*/



/* 
set_watchers() {

}

async watcher_signal() {
    /* when a signal is triggered  */
}

//launch at graceful stp 
async watcher_wait_before_kill {
    /* at stop, if the prcoess hasn't exited gracefully, KILL  */
}


async process_conclusion(AProcessOfTask processed_task) {
    /* is the return signal the right code? graceful exit ?*/

    // Check if the command was successful
    //if status.success() {
    //    println!("Command executed successfully");
    //} else {
    //    println!("Command failed with status: {}", status);
    //}
}
*/










#[cfg(test)]
mod process_test {

    use std::{collections::HashMap, process::Command, time::Duration};

    use tokio::time::sleep;

    use crate::{utils::process::ProcessOfTask, Task};

    /*pub  struct TaskMock {
        pub pgrm_name : String,
        pub cmd : Vec<String>,
        pub numprocs  : u32,
        pub umask  :  u32,
        pub workingdir  : PathBuf,
        pub autostart  : bool,
        pub autorestart  : String,
        pub exitcodes  : Vec<u32>,
        pub startretries  :  u32,
        pub starttime  : u32,
        pub stopsignal  : String,
        pub stoptime  :  u32,
        pub stdout  :  PathBuf,
        pub stderr  :  PathBuf,
        pub env  : HashMap<String, String>,
    }
    


    fn create_task_1() -> TaskMock {
        TaskMock { pgrm_name : String::from("nginxy"),
            cmd: vec!["ls".to_string(), "-la".to_string(), String::from("text"), "-x".to_string()],
            numprocs: 10 as u32,
            umask: 777 as u32,
            workingdir: PathBuf::from("./"),
            autostart: false,
            autorestart: String::from("true"),
            exitcodes: vec![254 as u32],
            startretries: 999_999_999 as u32,
            starttime: 999_999_999 as u32,
            stopsignal: String::from("SIGKILL"),
            stoptime: 999_999_999 as u32,
            stdout: PathBuf::from("./test_docs/parser_tests/file.stdout"),
            stderr: PathBuf::from("./test_docs/parser_tests/file.stderr"),
            env: HashMap::from([
                (String::from("STARTED_BY"), String::from("tasker")),
            ])
        }
    }

    fn assert_has_env_vars(process : Command) {
        let envs: Vec<(&OsStr, Option<&OsStr>)> = process.get_envs().collect();
        assert_eq!(envs, &[
            (OsStr::new("STARTED_BY"), Some(OsStr::new("tasker"))),
            (OsStr::new("TZ"), None)
        ]);
    }*/


    #[test]
    fn run_process() {

        let dev_log_out = String::from("./log/debug_stdout");
        let dev_log_err = String::from("./log/debug_stderr");

        //theory : Task struct exists but we wanna test independ.   the fns take a core struct , Task uses a to_that_core_struct() method to be used in this fn
        //let program_cmd_from_config_file = format!("export {} && umask {} && (cd {:?} && {}) >> {:?} 2>> {:?}");
        let task_forever_vec = Task::from_config(&String::from("./config/forever.sconfig")).unwrap();
        let task_forever = task_forever_vec.get(0).unwrap();
        //let task_read_dirs = Task::from_config("./config/only_read_dirs.sconfig").unwrap();
        //let task_ls3 = Task::from_config("./config/ls3.sconfig").unwrap();
        let mut process_forever = ProcessOfTask::new(task_forever, dev_log_out, dev_log_err);
        //let process_read_dirs = ProcessOfTask::new(&task_read_dirs);
        //let process_ls3 = ProcessOfTask::new(&task_ls3);
        process_forever.run(); //the handler field makes these asserts true 
        //assert!(process1.cmd , String::from(""));
        //assert!(process1.pid.is_some());
        assert!(process_forever.task_ref.pgrm_name == String::from("forever"));
        assert!(process_forever.is_running == true);
        sleep(Duration::from_secs(2));
        //process_forever.stop();
    }
}