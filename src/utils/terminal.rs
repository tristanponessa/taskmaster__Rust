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
    let all_tasks : Vec<Task>;
    let all_processes_of_tasks : Vec<ProcessOfTask>;
    let all_running_processes_of_tasks : Vec<ProcessOfTask>;

    let config_file = config_path.as_str();
    let all_tasks_res = Task::new_fetch_all(&String::from(config_file));
    if all_tasks_res.is_err() {
        panic!("{:?}", all_tasks_res.unwrap());
        
    }
    all_tasks = all_tasks_res.unwrap();
    all_processes_of_tasks = get_all_ProcessOfTask(all_tasks.clone());

    //now check which ones were changed compared to previous load , if so kill them
    //look at task env 
    //
    //
    //

    Taskmaster_Env {
        all_tasks : all_tasks.clone(),
        all_processes_of_tasks,
        all_running_processes_of_tasks : vec![],
    }
    

}



fn status(taskmaster_env : &Taskmaster_Env) {

    println!("{:?}", taskmaster_env.all_tasks);

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
        "status" => {
            status(&taskmaster_env);
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
        all_running_processes_of_tasks: vec![] 
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
