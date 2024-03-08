use std::fmt::format;
use std::os::unix::process::CommandExt;
use std::{clone, string, thread};
use std::collections::{HashMap};
use std::hash::Hash;
use std::io::Write;
use chrono::Local;
use nix::unistd::{getpid, getppid};

use std::fs::{read_to_string, File, OpenOptions};
use std::{env, io, path::{Path, PathBuf}, process::{exit, Command, ExitStatus, Stdio}}; //Path::new -> &Path plus needs Box<&Path> since it's unsized (don't implement Sized), Box or & or PathBuf(like an owned Path)  fixes it
use std::ffi::OsStr;
use std::time::Duration;
use std::thread::sleep;
use std::time::Instant;
use regex::Regex;
use std::process::{self, Child};

use nix::sys::signal::{self, Signal};

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



type BuildCmd = String;

#[derive(Debug)]
pub struct ProcessOfTask<'a> {
    handler: Command,
    child_handler : Option<Child>,
    task_ref : &'a Task,
    ppid : Option<u32>,
    pid: Option<u32>,
    final_exit_code: Option<i32>,
    nb_restarted : usize,
    last_error : Option<String>, 
    log_file_out : String, 
    log_file_err : String, 
}

impl<'a> ProcessOfTask<'a>{

    pub fn new(a_task : &'a Task, log_out : String, log_err : String) -> Self {
        Self {
            handler : Self::init_handler(a_task),
            child_handler : None, 
            task_ref : a_task,
            ppid : None,
            pid : None,
            nb_restarted : 0,
            final_exit_code: None,
            //is_running : false,
            last_error : None, 
            log_file_out : log_out,
            log_file_err : log_err,
        }
    }

    fn write_to_log_details(&mut self, which_file : &str, msg : String) {
        let now_date = Local::now();
        let self_details = format!("[PROGRAM {}] [PID {:?}]",self.task_ref.pgrm_name, self.pid);
        self.write_to_log(which_file, format!("[{}] {} {}",now_date, self_details, msg));
    }

    fn eprintln_details(&mut self, msg : String) {
        let now_date = Local::now();
        let self_details = format!("[PROGRAM {}] [PID {:?}]",self.task_ref.pgrm_name, self.pid);
        eprintln!("{}", format!("[{}] {} {}",now_date, self_details, msg));
    }
    

    fn write_to_log(&mut self, which : &str, data : String) {
        /* i can't write to log the open/write to log errors so i can only print them out on stderr */
        let data : String = data + "\n";
        let log_to_write = if which == "err" { &self.log_file_err } else { &self.log_file_out };
        let file_opened_res = OpenOptions::new()
                                                    .create(true)
                                                    .append(true)
                                                    .open(log_to_write);
        match file_opened_res {
            Ok(mut file_opened) => {
                let write_res = file_opened.write_all(data.as_bytes());
                match write_res {
                    Ok(_) => {
                        // Optional: Flush the buffer to ensure data is written immediately
                        let flush_res = file_opened.flush();
                        match flush_res {
                            Ok(_) => (),
                            Err(e) => self.eprintln_details(format!("flush failed : {}", e.to_string()))
                        }
                    }
                    Err(e) => self.eprintln_details(format!("write to file failed : {}", e.to_string()))
                }
                
            }
            Err(e) => self.eprintln_details(format!("file opened failed : {}", e.to_string()))
        }
    }

    //fn name_pid_str(&self) -> String {
    //    format!(" PROGRAM <{}> PID[{:?}] ",self.task_ref.pgrm_name, self.pid)
    //}

    /*pub fn run(&mut self) {
        self.init

    }*/

    fn pathBuf_to_File(p  :PathBuf) -> File {
        let s = String::from(p.to_str().unwrap());
        let file_path = Path::new(&s);
        File::open(file_path).unwrap()
    }

    pub fn init_handler(a_task : &Task) -> Command {
        let cmd = a_task.cmd.clone().remove(0);
        let mut args = a_task.cmd.clone();
        
        let mut handler = Command::new(cmd);
        handler.args(args);
        handler.current_dir(a_task.workingdir.clone());
        handler.stdout(Stdio::from(Self::pathBuf_to_File(a_task.stdout.clone()))); // Set stdout to capture command output
        handler.stderr(Stdio::from(Self::pathBuf_to_File(a_task.stderr.clone()))); // Set stderr to capture command error
            /* .pre_exec(|| {
                // Set umask to 022 (user has read, write, execute permissions) ONLY for this handler
                
                    libc::umask(a_task.umask); //0o022
                
                Ok(())
            });*/
        

        for (key, value) in &a_task.env {
            handler.env(key, value);
        }
        handler
    }


    fn is_process_running(&mut self, pid: u32) -> Option<bool> {
        // Run the "ps" command to list processes
        let output_res = Command::new("ps")
            .arg("-p")
            .arg(pid.to_string())
            .output();
        match output_res {
            Ok(output) => {      
                // Check if the output contains the header line and the process ID
                let output_str = String::from_utf8_lossy(&output.stdout);
                Some(output_str.contains("PID") && output_str.contains(&pid.to_string()))        
            },
            Err(e) => {
                self.write_to_log_details("err", format!("couldn't launch the to check if program is running : ps -p PID"));
                return None;
            }
        }
    }

    pub async fn run(&mut self) {
        /* runs and when takes care of exitcode */

        /* dont run if already running  if can't determine stop the program */
        /*if pid exists
            self.is_process_running(pid)
            if false continue (means it ran in the past )
            if true return early display already running 
        else
            continue
        */

        let cmd_spawned_res : io::Result<Child> = self.handler.spawn(); //blocks 
        match cmd_spawned_res {
            Ok(mut child) => {
                self.pid = Some(child.id());
                match self.pid {
                    Some(_) => {},
                    None => {
                        self.write_to_log_details("err", format!("couldn't retreive PID"));
                    }
                }
                let exit_status_res = child.wait();
                match exit_status_res {
                    Ok(exit_status) => {
                        let exit_code_opt = exit_status.code();
                        match exit_code_opt {
                            Some(exit_code) => {
                                self.final_exit_code = Some(exit_code);
                                self.write_to_log_details("out", format!("exitcode : {:?}", exit_code));
                                //compare to self.task_ref.exitcodes and print if existed gracefully 
                            },
                            None => {
                                self.write_to_log_details("err", format!("couldn't get exitcode"));
                            }
                        }
                    },
                    Err(e) => {
                        self.write_to_log_details("err", format!("retreiving exitcode error : {:?}", e.to_string()));
                    }
                }
                self.child_handler = Some(child);
            }
            Err(e) => self.write_to_log_details("err", format!("{} [{:?}] : {}", String::from("command run failed :"), self.task_ref.cmd,e.to_string())),
        }    




        
    }

    //// Send SIGTERM to child process.
    //signal::kill(Pid::from_raw(child.id()), Signal::SIGTERM).unwrap();
    pub async fn graceful_stop_timeout_kill(&mut self) {
        match self.child_handler {
            Some(ref mut child) => {
                sleep(Duration::from_secs(self.task_ref.stoptime.into()));
                let kill_res = child.kill(); //SIGKILL
                match kill_res {
                    Ok(_) => {
                        self.write_to_log_details("out", format!("killed, graceful stop failed to execute before timeout : {}", self.task_ref.stoptime));
                    },
                    Err(e) => {
                        self.write_to_log_details("err", format!("stop attempt error : {:?}, possibly cause the program stopped gracefully", e.to_string()));
                    }
                }       
            },
            None => {
                self.write_to_log_details("err", format!("can't proform kill command, possibly cause process hasn't been launched"));
            }
        }
    }

    
    


}



#[test]
fn process1() {

    let dev_log_out = String::from("./log/debug_stdout");
    let dev_log_err = String::from("./log/debug_stderr");

    //theory : Task struct exists but we wanna test independ.   the fns take a core struct , Task uses a to_that_core_struct() method to be used in this fn
    //let program_cmd_from_config_file = format!("export {} && umask {} && (cd {:?} && {}) >> {:?} 2>> {:?}");
    let task_forever_vec = Task::from_config(&String::from("./config/forever.sconfig")).unwrap();
    let task_forever : &Task = task_forever_vec.get(0).unwrap();
    //let task_read_dirs = Task::from_config("./config/only_read_dirs.sconfig").unwrap();
    //let task_ls3 = Task::from_config("./config/ls3.sconfig").unwrap();
    let mut process_forever = ProcessOfTask::new(task_forever, dev_log_out, dev_log_err);
    //let process_read_dirs = ProcessOfTask::new(&task_read_dirs);
    //let process_ls3 = ProcessOfTask::new(&task_ls3);
    //process_forever.run(); //the handler field makes these asserts true 
    //assert!(process1.cmd , String::from(""));
    //assert!(process1.pid.is_some());
    //assert!(process_forever.task_ref.pgrm_name == String::from("forever"));
    //assert!(process_forever.is_running == true);
    //sleep(Duration::from_secs(2));
    //process_forever.stop();
    //assert!(process_forever.is_running == false);
}
