use std::{io::{stdout, stdin, Write, Error}, process::Command, path::Path, env};

//from what iunderstand a cmd is without piping &&  || | >>  << on   supervisor official doc examples and github repos  

fn cmd_help(user_cmds : &[&str]) {
    eprintln!("help: {:?}", user_cmds);
}

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


pub async fn run_terminal() ->  Result<(),Error> {

    //let cur_config : Option<ConfigParser>  = None;
    let user_cmds = ["help", "config", "load", "start", "restart", "stop", "status", "exit"];

    loop {
        print!("> ");
        if let Err(e) = stdout().flush() {
            eprintln!("ERROR: failed to flush '>' : {}", e);
            return Err(e);
        }

        let mut input = String::new();
        if let Err(e) = stdin().read_line(&mut input) {
                eprintln!("ERROR: failed to read line : {}", e);
                return Err(e);
        }

        //Config::parser cmd_args
        let parts : Vec<&str>  = input.trim().split_whitespace().collect();
		if parts.len() == 0 {
			continue;
		}
        let command = *parts.first().unwrap(); //will always  succeed , handled  by prior if condition
        let args = match &parts {
            p if p.len() > 1 => &p[1..],
            _ => &parts[..],
        };

        //let command = parts.collect();
        //let command = parts.next().unwrap();
        //let args = parts;

        match command {
           


            
            
            
            

            //"load" => cmd_load(args);
            //"status" => {},
            //"start" => {},
            //"stop" => {},
            //"restart" => {},
           //"config" => eprintln!("cur Config : {:?}", cur_config),
            "help" => cmd_help(&user_cmds),
            "exit" => return Ok(()),
            _ => { 
                eprintln!("cmd '{}' don't exist", command);
                cmd_help(&user_cmds);
            }         
            
            
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
