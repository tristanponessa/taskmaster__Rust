use std::{io::{stdout, stdin, Write, Error}, process::Command, path::Path, env};
use crate::*;


//mod self::utils;
//from what iunderstand a cmd is without piping &&  || | >>  << on   supervisor official doc examples and github repos  



//fn cmd_load(args: &[&str]) -> Result<ConfigParser> {

    //"load" => {
                    //let r = parser_main();
                /*

                    //reload
                    match cur_config {
                        Some(x) => { },
                        None => { },
                    }

                    let cur_config = match &r {
                        Ok(r) => { 
                            eprintln!("new Config file loaded : {:?}", r),
                            //pgrms display what shut off changed
                        },
                        Err(e) => {
                            eprintln!("loading Config file failed : {:?}", e),
                            eprintln!("cur Config : {:?}", cur_config),
                        }
                    };*/
//}






type LoopSignal = String;
pub struct Terminal_Cmd {
    cmd : String,
    args : Vec<String>
}


fn flush_stdout() {
    if let Err(e) = stdout().flush() {
        panic!("ERROR: failed to flush '>' : {}", e);
        //return Err(e);
    }
}




fn load_config(taskmaster_env : Taskmaster_Env, config_path : String) -> Taskmaster_Env {
    //PROCESSES CONTROLLER 
    let mut all_tasks : Vec<Task>;
    let mut all_processes_of_tasks : Vec<ProcessOfTask>;
    //let all_running_processes_of_tasks : Vec<ProcessOfTask>;

    let config_file = config_path.as_str();
    let all_tasks_res = Task::new_fetch_all(&String::from(config_file));
    match all_tasks_res {

        Ok(all_tasks) => {
            all_processes_of_tasks = get_all_ProcessOfTask(all_tasks.clone());
            Taskmaster_Env {
                all_tasks : all_tasks.clone(),
                all_processes_of_tasks,
                //all_running_processes_of_tasks : vec![],
            }
        }
        Err(e) => { 
            println!("{:?}", e);
            taskmaster_env
        }
        
    
        
    }

    //now check which ones were changed compared to previous load , if so kill them
    //look at task env 
    //
    //
    //

    
    

}



fn status_debug(taskmaster_env : &Taskmaster_Env) {

    println!("{:?}", taskmaster_env.all_tasks);
    println!("{:?}", taskmaster_env.all_processes_of_tasks);
    //println!("{:?}", taskmaster_env.all_running_processes_of_tasks);

}

fn get_user_input() -> Terminal_Cmd {
    let mut input : String;
    
    input = String::new();
        if let Err(e) = stdin().read_line(&mut input) {
                panic!("ERROR: failed to read line : {}", e);
                //return Err(e);
        }
        let parts : Vec<&str>  = input.trim().split_whitespace().collect();
        let command = *parts.first().unwrap(); //will always  succeed , handled  by prior if condition
        let args = match &parts {
            p if p.len() > 1 => &p[1..],
            _ => &parts[..],
        };
        Terminal_Cmd {
            cmd : String::from(command),
            args : args.iter().map(|&s| s.to_string()).collect()
        }
}

fn get_program_names(taskmaster_env : &Taskmaster_Env) -> Vec<String> {
    taskmaster_env.all_tasks.iter().map(|a_task : &Task| a_task.pgrm_name.clone()).collect()
}

fn get_index_task_and_processOfTask_for_program(taskmaster_env : &Taskmaster_Env, program_name : String) -> Option<usize> {
    let indx : Vec<usize> = taskmaster_env.all_tasks.iter()
                            .enumerate()
                            .filter_map(|(idx, a_task)| 
                                    if a_task.pgrm_name.clone() == program_name {Some(idx)} else {None})
                            .collect();
    if indx.len() > 0 {
        return Some(*indx.get(0).unwrap()); //take any
    }
    None    
}
/*fn get_task_and_processOfTask_for_program(taskmaster_env : &Taskmaster_Env, program_name : String) -> (Task, ProcessOfTask) {
    for (a_task, a_task_of_process) in taskmaster_env.all_tasks.iter().zip(taskmaster_env.all_processes_of_tasks.iter()) {
        
    }
}*/





fn handle_user_input(taskmaster_env : Taskmaster_Env, possible_cmds : &Vec<&str>, user_input : Terminal_Cmd) -> (LoopSignal, Taskmaster_Env) {
    match user_input.cmd.as_str() {
           
        "load" => {
            if user_input.args.len() == 1 {
                let config_path = user_input.args.get(0).unwrap();
                let taskmaster_env = load_config(taskmaster_env , config_path.clone());
                return (String::from("continue"), taskmaster_env);
            }
            (String::from("continue"), taskmaster_env)
        }
        "run" => {
            if user_input.args.len() == 1 {
                let input_program_name = user_input.args.get(0).unwrap();
                let idx : Option<usize>  = get_index_task_and_processOfTask_for_program(&taskmaster_env, input_program_name.clone());
                match idx {
                    Some(idx) => {
                        let mut processOfTask_for_program: ProcessOfTask;
                        //for i in 0..idx {
                        processOfTask_for_program = *taskmaster_env.all_processes_of_tasks.get(idx).unwrap();
                        //}
                        run_process_of_task(&mut processOfTask_for_program);
                        return (String::from("continue"), taskmaster_env);
                        

                        //let processOfTask_for_program : &mut ProcessOfTask = &mut taskmaster_env.all_processes_of_tasks.into_iter().nth(idx).unwrap(); //get(idx).unwrap();
                        
                        
                    },
                    None => {
                        let program_names = get_program_names(&taskmaster_env);
                        println!("{} program don't exist, ones that do : {:?}", input_program_name, program_names);
                    }
                }
                return (String::from("continue"), taskmaster_env);
            }
            //if none run all
            (String::from("continue"), taskmaster_env)
        }
        "status_debug" => {
            status_debug(&taskmaster_env);
            (String::from("continue"), taskmaster_env)
        },
        //"start" => {},
        //"stop" => {},
        //"restart" => {},
       //"config" => eprintln!("cur Config : {:?}", cur_config),
        "help" => {
            cmd_help(&possible_cmds);
            (String::from("continue"), taskmaster_env)
        }
        "exit" => {

            (String::from("stop"), taskmaster_env)
        }
        _ => { 
            eprintln!("cmd '{}' don't exist",  user_input.cmd);
            cmd_help(&possible_cmds);
            (String::from("continue"), taskmaster_env)
        }            
    }
}

fn cmd_help(possible_cmds : &Vec<&str>) {
    eprintln!("help: {:?}", possible_cmds);
}


pub fn run_terminal() {


    let mut taskmaster_env : Taskmaster_Env = Taskmaster_Env { all_tasks: vec![], 
        all_processes_of_tasks: vec![], 
        //all_running_processes_of_tasks: vec![] 
    };
    let mut loop_signal : LoopSignal;

    

    //TERMINAL LOOP 

    let possible_cmds = vec!["help", "config", "load", "start", "restart", "stop", "status", "exit"];
    loop {
        print!("> ");
        flush_stdout();
        let user_input = get_user_input();
        if user_input.cmd == "" {
            continue;
        }
        (loop_signal, taskmaster_env ) = handle_user_input(taskmaster_env,&possible_cmds ,user_input);
        if loop_signal == "stop" {
            break;
        }
    }
}



//for any cmd
            /*command => {
                //
                let child = Command::new(command)
                    .args(args)
                    .spawn();

                match child {
                    Ok(mut child) => { 
                        match child.wait() {
                            Ok(r) => eprintln!("command: '{}' finished with exitstatus : {}", command, r),
                            Err(e) => eprintln!("ERROR: command: '{}' failed to retreive exitstatus! {}", command, e),
                        };
                    },
                    Err(_) => { 
                        eprintln!("cmd '{}' don't exist", command);
                        cmd_help(&possible_cmds);
                    }                    
                };
            }*/ 
            
            /*"cd" => {
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            },*/
