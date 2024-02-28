use utils::terminal::run_terminal;

mod utils;


//launch cmd 
use std::{env, process::{Command, Stdio}};

LST PRS STATUS AND CONFIG 
FOR EVERY CMD (CMD * NB PROCESSUS COUNTS AS N CMDS )
    1.PURE CMDS   cd env  >> stdout file 
    2.WATCHERS timers  reruns   signals 
    3.CHECKERS for exit    see if exitcodes right 

! hcnage ConfigParser to ATask



//were gonaj do PURE FNS   if one elem has ot be chane copy entire lst and change that one elem  OR spread data ascross mulltiple vecs 


type EnvCmd = String;
type ConfigParser<Task> = Vec<Task>;

//shouldi use impl instead Self::global
static mut ALL_TASKS : Vec<Task> = vec![];
static mut ALL_PROCESS_OF_TASKS : Vec<ProcessOfTask> = vec![];

#[derive(Debug)]
struct ProcessOfTask {
    handler: Command,
    task_name : String,
    pid: Option<usize>,
    final_exit_code: Option<usize>,
    nb_restarted : usize,
}

fn vec_ProcessOfTask_new(all_tasks : Vec<Task>) -> Vec<ProcessOfTask>  {
    for a_task in all_tasks.iter() {
        for i in a_task.numprocs {
            let env_cmd = env_setup_str(a_task);
            let new_ProcessOfTask = create_process_of_task(env_cmd , a_task); //OR AProcessOfTask.new(atask)
            ALL_PROCESS_OF_TASKS.push(new_ProcessOfTask);
        }
    }
}

run_processes (vec<AProcessOfTask>) {
    FOR AProcessOfTask IN vec<AProcessOfTask> {
        LAUNCH_processes 
    }
}

fn env_setup_str(a_task : Task) -> EnvCmd {

    //A = B , C = D hashmap
    let env_cmd : String = a_task.env
        .iter()
        .map(|(key, value)| format!(" {}={} ", key, value)) // Add 10 to each value
        .collect();
    format!("export {} && cd {} && umask {} && {} >> {} 2>> {}", env_cmd
                                a_task.workingdir, 
                                a_task.umask,
                                a_task.cmd,
                                a_task.stdout,
                                a_task.stderr)
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

fn run_process_of_task(process_task : ProcessOfTask) ->  ProcessOfTask {
    let exit_status : io::Result<ExitStatus> = processed_task.handler.status().expect("Failed to execute command");
    let exit_code : Option<usize> = if exit_status.is_None() { None } else  { Some(exit_status.unwrap().code()) }; //if fails immediatly 
    ProcessOfTask {
        handler : command,
        pid : None,
        nb_restarted : 0,
        final_exit_code: exit_code,
        task_name : a_task.pgrm_name
    }
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
    run_terminal();
}
