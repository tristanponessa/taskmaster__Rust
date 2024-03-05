use std::{env, io, path::{Path, PathBuf}, process::{exit, Command, ExitStatus, Stdio}};
//use utils::terminal::run_terminal;
use std::time::Instant;
use utils::config_parser::*;
mod utils;


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




/* 
 fn watcher_time_ran() {

}

 fn _timer_example(FN TO WATCH) {
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

 watcher_signal() {
    /* when a signal is triggered  */
}

//launch at graceful stp 
 watcher_wait_before_kill {
    /* at stop, if the prcoess hasn't exited gracefully, KILL  */
}


 process_conclusion(AProcessOfTask processed_task) {
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
   
    
    
    //RUN SYNCHR.
    //all_running_processes_of_tasks = run_processes_of_tasks(all_processes_of_tasks);
    /*for running_process_of_task in all_running_processes_of_tasks.iter() {
        println!("running > {:?}", running_process_of_task.pid);
    }*/
    

    //run_terminal();
}
















/*fn make_relative_path(from: &Path, to: &Path) -> String {
    // Compute the relative path from 'from' to 'to'
    let relative_path = to.strip_prefix(from)
        .unwrap_or_else(|_| to)
        .to_str().unwrap();


    "./".to_owned() + relative_path
}*/

/* 
fn cd_into_dir(wd : &PathBuf) {
    env::set_current_dir(wd).expect("couldn't change dirs");
}

fn get_cur_dir() -> PathBuf {
    if let Ok(cwd) = env::current_dir() {
      cwd  
    } else {
        panic!("Failed to get current working directory");
    }
}
use std::fs;
fn get_absolute(path : &str) -> String {
    // File or directory path
    // Get the absolute path
    if let Ok(absolute_path) = fs::canonicalize(path) {
        println!("Absolute path: {:?}", absolute_path);
        String::from(absolute_path.to_str().unwrap())
    } else {
        panic!("Failed to get absolute path");

    }
}*/