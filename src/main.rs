use std::{env, io, process::{exit, Command, ExitStatus, Stdio}};
use utils::terminal::run_terminal;
use utils::config_parser::*;
mod utils;


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


type EnvCmd = String;
//type ConfigParser<Task> = Vec<Task>;

//shouldi use impl instead Self::global
//static mut ALL_TASKS : Vec<Task> = vec![];
//static mut ALL_PROCESS_OF_TASKS : Vec<ProcessOfTask> = vec![];

#[derive(Debug)]
struct ProcessOfTask {
    handler: Command,
    task_name : String,
    pid: Option<u32>,
    final_exit_code: Option<i32>,
    nb_restarted : usize,
}

fn get_all_ProcessOfTask(all_tasks : Vec<Task>) -> Vec<ProcessOfTask>  {
    let mut all_processes_of_tasks : Vec<ProcessOfTask> = vec![];
    for a_task in all_tasks.iter() {
        for i in 0..a_task.numprocs {
            let env_cmd = env_setup_str(a_task.clone());
            let new_ProcessOfTask = create_process_of_task(env_cmd , a_task.clone()); //OR AProcessOfTask.new(atask)
            all_processes_of_tasks.push(new_ProcessOfTask);
        }
    }
    all_processes_of_tasks
}

fn run_processes_of_tasks(all_processes_of_tasks : Vec<ProcessOfTask>) {
    for a_process_of_task in all_processes_of_tasks.iter() {
        println!("wanna run > {}", a_process_of_task.task_name);
        //
    }
}

fn env_setup_str(a_task : Task) -> EnvCmd {

    //A = B , C = D hashmap
    let env_cmd : String = a_task.env
        .iter()
        .map(|(key, value)| format!(" {}={} ", key, value)) // Add 10 to each value
        .collect();
    format!("export {} && cd {:?} && umask {} && {} >> {:?} 2>> {:?}", env_cmd,
                                a_task.workingdir.to_str(), 
                                a_task.umask,
                                a_task.cmd.join(" "),
                                a_task.stdout.to_str(),
                                a_task.stderr.to_str())
}



fn create_process_of_task(env_cmd : EnvCmd , a_task : Task) -> ProcessOfTask {

    // Create a new Command
    let mut command = Command::new("bash");

    // Change directory using `cd`
    command.arg("-c").arg(env_cmd);

    // Set the command's standard output to be inherited by the parent process
    command.stdout(Stdio::inherit());

    // Set the command's standard error to be inherited by the parent process
    command.stderr(Stdio::inherit());

    ProcessOfTask {
        handler : command,
        pid : None,
        nb_restarted : 0,
        final_exit_code: None,
        task_name : a_task.pgrm_name
    }
}


fn run_process_of_task(mut process_task : ProcessOfTask) -> Result<ProcessOfTask, io::Error> {
    /* the process is not running, once you run it, it will have a pid , and the other fields will be updated  */
    //we dont want to modify the old one so we clone   but it dont implement it ......

    //RUNS status() runs cmd
    let exit_status : io::Result<ExitStatus> = process_task.handler.status(); 
    let exit_status_code = exit_status?.code();
    //let exit_code : Option<usize> = if exit_status.is_ok() { Some(exit_status.unwrap().code().unwrap()) } else  { None }; //if fails immediatly 
    Ok(ProcessOfTask {
        handler : process_task.handler,
        pid : Some(std::process::id()), //gets current running process ... lets hope it refers this one 
        nb_restarted : 0,
        final_exit_code: exit_status_code,
        task_name : process_task.task_name
    })
}

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












fn main() {
   
    let all_tasks : Vec<Task>;
    let all_processes_of_tasks : Vec<ProcessOfTask>;

    let config_file = "./config/def.sconfig";
    let all_tasks_res = Task::new_fetch_all(&String::from(config_file));
    if all_tasks_res.is_err() {
        println!("{:?}", all_tasks_res.unwrap());
        exit(0);
    }
    all_tasks = all_tasks_res.unwrap();
    all_processes_of_tasks = get_all_ProcessOfTask(all_tasks);
    run_processes_of_tasks(all_processes_of_tasks);


    

   
   
   
   // run_terminal();
}
