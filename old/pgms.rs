use std::{process::{Command, Stdio, Child}, fs::File, io::Error};

use super::parser::ConfigParser;


//global fns  //this makes pgms  dependant of parser::ConfigParser
//shouldi use impl instead Self::global
static mut CONFIG_PARSER_VEC : Vec<ConfigParser> = vec![];
//static mut PGM_HANDLERS : Vec<Child> = vec![];
static mut PGM_STATES : Vec<pgm_State> = vec![];

#[derive(Debug)]
//used from status 
struct pgm_State {
    obj: Option<Command>,
    handler: Option<Result<Child,Error>>,
    name : Option<String>,
    id: Option<usize>,
    final_exit_code: Option<usize>,
    nb_restarted : Option<usize>,
    time_elapsed: Option<String>,
}
impl pgm_State {
    fn new() -> pgm_State {
        pgm_State {
           obj : None,
           handler: None,
           name : None,
           id : None,
           final_exit_code : None,
           nb_restarted : None,
           time_elapsed : None,
        }
    }
}

/* do it yourself
impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
         .field("x", &self.x)
         .field("y", &self.y)
         .finish()
    }
}
*/


//clean up



pub fn pgm_main() {
    //add to globals 
    //dealwith errors
}

fn open_file_mode_append_as_stdio(path: &str) -> Stdio {
    //for std::Process::cmd stdout stderr option
    let ff =  File::options().append(true).open(path).unwrap();
    let stdio = Stdio::from(ff);
    stdio
}





//FOREACH PRGM
//shadows operation push to global CANT TEST CAUSE DEPENDANT!
//instead doit in outside fn that isnt tested 
//global_pgm_handlers : &Vec<Child>, 


fn setup_pgm(cp: &ConfigParser) -> pgm_State {

    //set umask with pipe or set in args somehow
    let mut pgm_handler : Command = Command::new(cp.cmd[0].clone());
    pgm_handler.current_dir(cp.workingdir.as_path()); //might of moved
    if cp.cmd.len() > 1 {
        pgm_handler.args(&cp.cmd[1..]);
    }
    /*if cp.env.len() > 0 {
        pgm_handler.envs(cp.env);
    }*/
    //if cp.stdout != "none"
    pgm_handler.stdout(open_file_mode_append_as_stdio(cp.stdout.to_str().unwrap()));//or file?
    pgm_handler.stderr(open_file_mode_append_as_stdio(cp.stderr.to_str().unwrap()));
    //pgm_handler.spawn();    //.output()could be used but waits for it to finish             
    //.expect("ERROR: failed to execute process");            
    //global_pgm_handlers.push(pgm_handler); //move

    let mut state = pgm_State::new();
    state.obj = Some(pgm_handler);
    state.name = Some(cp.pgrm_name.clone());

    return state;

}
//get current  output


//get  exit code 



/*
assert!(output.status.success());
thread::sleep(Duration::from_secs(5));
                cmd_handler.kill().expect("command wasn't running");
*/


#[cfg(test)]
mod tests {
    use std::{path::PathBuf, collections::HashMap};

    use super::*;

    fn  assert_eq_pgmState (pg1 : pgm_State , pg2 : pgm_State) {
        //cmp Command?
        //cmp Result<Child>?

        assert_eq!(pg1.name,pg2.name);
        assert_eq!(pg1.obj.unwrap().get_program(), pg2.obj.unwrap().get_program());

    }

    #[test]
    fn test_setup_pgm() {
        let expected_ConfigParser_1 : ConfigParser = ConfigParser {
            pgrm_name: String::from("LS"),
            cmd: vec!["ls".to_string()],
            numprocs: 1 as u32,
            umask: 22 as u32,
            workingdir: PathBuf::from("./"),
            autostart: true,
            autorestart: String::from("unexpected"),
            exitcodes: vec![0 as u32,2 as u32],
            startretries: 3 as u32,
            starttime: 5 as u32,
            stopsignal: String::from("SIGTERM"),
            stoptime: 10 as u32,
            stdout: PathBuf::from("./test_docs/parser_tests/file.stdout"),
            stderr: PathBuf::from("./test_docs/parser_tests/file.stderr"),
            env: HashMap::from([
                (String::from("STARTED_BY"), String::from("taskmaster")),
                (String::from("ANSWER"), String::from("42")),
            ])
        };

        let mut expected_pgm_handler_1 : Command = Command::new("ls");
        expected_pgm_handler_1.current_dir("./")
        .envs(HashMap::from([
            (String::from("STARTED_BY"), String::from("taskmaster")),
            (String::from("ANSWER"), String::from("42")),
        ]))
        .stdout(open_file_mode_append_as_stdio("./test_docs/parser_tests/file.stdout"))
        .stderr(open_file_mode_append_as_stdio("./test_docs/parser_tests/file.stderr"));

        let expected_pgm_State = pgm_State {
            obj : Some(expected_pgm_handler_1),
            handler: None,
            name : Some(String::from("LS")),
            id : None,
            final_exit_code : None,
            nb_restarted : None,
            time_elapsed : None,
        };

        let res = setup_pgm(&expected_ConfigParser_1); 

        assert_eq_pgmState(res,expected_pgm_State);


    }
}