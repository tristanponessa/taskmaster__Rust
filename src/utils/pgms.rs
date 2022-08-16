use std::process::Command;


//global fns  //this makes pgms  dependant of parser::ConfigParser
//shouldi use impl instead Self::global
static mut CONFIG_PARSER_VEC : Vec<ConfigParser> = vec![];
static mut PGM_HANDLERS : Vec<Child> = vec![];
static mut PGM_STATES : Vec<pgm_State> = vec![];

#[derive(Debug)]
//used from status 
struct pgm_State {
    name : String,
    id: usize,
    final_exit_code: usize,
    nb_restarted : usize,
    time_elapsed: String,
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


//FOREACH PRGM
//shadows operation push to global
fn start_pgm(cp: &ConfigParser) -> Result<Child> {

    //set umask with pipe or set in args somehow
    let pgm_handler : Result<Child> =  
        Command::new(cp.cmd[0])
                .args(&cp.cmd[1..]) //!
                .current_dir(cp.workingdir)
                .envs(cp.env)
                .stdout(Stdio::from(cp.stdout))//or file?
                .stderr(Stdio::from(cp.stderr))
                .spawn();    //.output()could be used but waits for it to finish             
                //.expect("ERROR: failed to execute process");
    return pgm_handler;
    PGM_HANDLERS.push(pgm_handler); //move

}
//get current  output


//get  exit code 


println!("status: {}", output.status);
io::stdout().write_all(&output.stdout).unwrap();
io::stderr().write_all(&output.stderr).unwrap();

assert!(output.status.success());
thread::sleep(Duration::from_secs(5));
                cmd_handler.kill().expect("command wasn't running");


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
}