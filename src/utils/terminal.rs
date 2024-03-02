use std::{io::{stdout, stdin, Write, Error}, process::Command, path::Path, env};

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
struct Terminal_Cmd {
    cmd : String,
    args : Vec<String>
}


fn flush_stdout() {
    if let Err(e) = stdout().flush() {
        panic!("ERROR: failed to flush '>' : {}", e);
        //return Err(e);
    }
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


fn handle_user_input(user_cmds : &Vec<&str>, user_input : Terminal_Cmd) -> LoopSignal {
    match user_input.cmd.as_str() {
           
        //"load" => cmd_load(args);
        //"status" => {},
        //"start" => {},
        //"stop" => {},
        //"restart" => {},
       //"config" => eprintln!("cur Config : {:?}", cur_config),
        "help" => {
            cmd_help(&user_cmds);
            String::from("continue")
        }
        "exit" => String::from("stop"),
        _ => { 
            eprintln!("cmd '{}' don't exist",  user_input.cmd);
            cmd_help(&user_cmds);
            String::from("continue")
        }            
    }
}

fn cmd_help(user_cmds : &Vec<&str>) {
    eprintln!("help: {:?}", user_cmds);
}


pub fn run_terminal() {
    let user_cmds = vec!["help", "config", "load", "start", "restart", "stop", "status", "exit"];
    loop {
        print!("> ");
        flush_stdout();
        let user_input = get_user_input();
        if user_input.cmd == "" {
            continue;
        }
        let loop_signal = handle_user_input(&user_cmds ,user_input);
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
                        cmd_help(&user_cmds);
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
