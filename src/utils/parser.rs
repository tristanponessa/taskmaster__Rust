
use std::collections::{HashMap};
/*
    add stdout stderr : sys stdout insteadof file 
*/


use std::fs::{File, read_to_string};
use std::path::{Path, PathBuf}; //Path::new -> &Path plus needs Box<&Path> since it's unsized (don't implement Sized), Box or & or PathBuf(like an owned Path)  fixes it
use std::ffi::OsStr;
use regex::Regex;




#[derive(PartialEq,Debug,Clone)] //used for tests   Debug so cargo test can display if assert fails   Clone  formultiple borrows
pub  struct ConfigParser {
    pub pgrm_name : String,
    pub cmd : Vec<String>,
    pub numprocs  : u32,
    pub umask  :  u32,
    pub workingdir  : PathBuf,
    pub autostart  : bool,
    pub autorestart  : String,
    pub exitcodes  : Vec<u32>,
    pub startretries  :  u32,
    pub starttime  : u32,
    pub stopsignal  : String,
    pub stoptime  :  u32,
    pub stdout  :  PathBuf,
    pub stderr  :  PathBuf,
    pub env  : HashMap<String, String>,
}

//Limits : all global data 
struct Limits {
    //cmp with ConfigParser inst.
    //nb_prgms : Regex, //anti DoS atk
    //pgrm_name : Regex,
    //cmd : Regex, //anything between  
    //numprocs  : u32, //anti DoS atk
    //umask  : u32,
    //exitcodes  : u32,
    //startretries  : u32,
    //starttime  : u32,
    stopsignal  : [String; 13],
    //stoptime  : u32,
    //env  : Regex,
}

pub fn get_pgrm<'a>(v:&'a Vec<ConfigParser>, name: &'a str) -> Option<&'a ConfigParser> {
    for p in v {
        if p.pgrm_name == name {
            Some(p);
        }
    }
    None
}

/*originiallyfor cmd  check
struct Easy_Regex;
impl Easy_Regex{
    fn easy_regex(s: String) -> bool {
        let banned = [" && ", " | ", " ; ", '']
        let in_pars = false;

    }
}
*/

//test this fn


impl ConfigParser {

    //you have tol detail all fields and vals so this is usuelss
    /*fn new_default() -> Self {
        
        Self { 
            pgrm_name : String::from("default_prgm"),
            cmd  : String::from("ls -la ."),
            numprocs  : 1,
            umask  : 0o777,
            workingdir  : Path::new("/tmp"),
            autostart  : true,
            autorestart  : false,
            exitcodes  : vec![0,1],
            startretries  : 3,
            starttime  : 15,
            stopsignal  : String::from("TERM"),
            stoptime  : 10,
            stdout  : Path::new("/tmp/ls.stdout"),
            stderr  : Path::new("/tmp/ls.stderr"),
            env  : HashMap::from([("STARTED_BY :taskmaster"),("author","trponess")]),
        }
    }*/

    //we dont need to check nb threw int, regex does all the work
    fn new_limits() -> Limits {
        Limits {
            //nb_prgms : 10, //anti DoS atk
            //pgrm_name : Regex::new(r"^pgrm_name > [a-zA-Z_]+$").unwrap(),
            //cmd : Regex::new(r#""([^;]*)""#).unwrap(), //anything between " "
            //numprocs  : 10, //anti DoS atk
            //umask  : 0o777,
            //exitcodes  : u32::MAX,
            //startretries  : u16::MAX,
            //starttime  : u16::MAX,
            stopsignal  : [
                String::from("SIGHUP"), //1
                String::from("SIGINT"), //2
                String::from("SIGQUIT"), //3
                String::from("SIGILL"), //4
                String::from("SIGTRAP"), //5
                String::from("SIGABRT"), //6
                String::from("SIGBUS"), //7
                String::from("SIGFPE"), //8
                String::from("SIGKILL"), //9
                String::from("SIGUSR1"), //10
                String::from("SIGSEGV"), //11
                String::from("SIGALRM"), //14
                String::from("SIGTERM"), //15
            ],//array of all 16 signals morebut 16 is used for some reason 
            //stoptime  : 10,
            //env  : Regex::new(r"^[a-zA-Z_]+=[a-zA-Z0-9 ]+$").unwrap()
        }
    }

    fn get_regex(which: &str) -> &str {
        //each nb corresponds to the line it has ot be on
        match which{
            "prgm_name" | "0" => r"^prgm_name: [a-zA-Z_0-9]+$",
            "cmd" | "1"  => "^cmd: ([^;]*)$",
            "numprocs" | "2"   => r"^numprocs: (10|[1-9])$",
            "umask" | "3"  => r"^umask: [0-7]{3}$",
            "workingdir" | "4" => r"^workingdir: [a-zA-Z0-9._/]+$",
            "autostart" | "5"  => r"^autostart: (true|false)$",
            "autorestart" | "6"  => r"^autorestart: (true|false|unexpected)$",
            "exitcodes" | "7"  => r"^exitcodes: ([0-9]+,)+$",
            "startretries" | "8"  => r"^startretries: [0-9]{1,9}$",
            "starttime" | "9"  => r"^starttime: [0-9]{1,9}$",
            "stopsignal" | "10"  => r"^stopsignal: [A-Z]+$",
            "stoptime" | "11"  => r"^stoptime: [0-9]{1,9}$",
            "stdout" | "12"  => r"^stdout: [a-zA-Z0-9._/]+$",
            "stderr" | "13"  => r"^stderr: [a-zA-Z0-9._/]+$",
            "env" | "14"  => r"^env: ([A-Z]+[A-Z_]+=[a-zA-Z0-9_]+,)+$",
            &_  => "",
        }
    }

    //utility fns
    fn is_regex(reg : &str , s : &String) -> bool {
        let re = Regex::new(reg).unwrap();
        re.is_match(s)
    }

    fn explode(s : &String , token : String) -> (String, String)  {
        let g : Vec<&str> = s.split(&token).collect::<Vec<&str>>(); //collect don't have Vec<String> impl
        let p1 = String::from(*(g.get(0).unwrap_or(&""))); //get returns &&str
        let p2 = String::from(*(g.get(1).unwrap_or(&"")));
        (p1, p2)
    }

    /*
    fn cmd_parse(cmd: String) -> Result<> {
        //crate clap 
        use clap::App;
        let words = shellwords::split(cmd)?;
        let matches = App::new("taskmaster_app").get_matches_from(words);

    }*/

    fn hashmap_to_ConfigParser(m : &HashMap<String, String>) -> ConfigParser {
        //deal with unwrap better
        
        //env 
        let mut env  : HashMap<String, String> = HashMap::new();
        let parts : Vec<_> = m.get("env").unwrap().split(",").collect();
        for p in parts {
            let (var, val) = Self::explode(&String::from(p), String::from("="));
            if var != "" {
                env.insert(var, val);
            }
        }

        ConfigParser {
            pgrm_name : m.get("prgm_name").unwrap().to_string(),
            cmd : shellwords::split(m.get("cmd").unwrap()).unwrap(),
            numprocs: m.get("numprocs").unwrap().parse::<u32>().unwrap(),
            umask: m.get("umask").unwrap().parse::<u32>().unwrap(),
            workingdir: PathBuf::from(m.get("workingdir").unwrap()),
            autostart: if m.get("autostart").unwrap() == "true" {true} else {false},
            autorestart: m.get("autorestart").unwrap().to_string(),
            exitcodes: m.get("exitcodes").unwrap().split(",").filter(|e| *e != "").map(|e| e.parse::<u32>().unwrap()).collect(),
            startretries: m.get("startretries").unwrap().parse::<u32>().unwrap(),
            starttime: m.get("starttime").unwrap().parse::<u32>().unwrap(),
            stopsignal: m.get("stopsignal").unwrap().to_string(),
            stoptime: m.get("stoptime").unwrap().parse::<u32>().unwrap(),
            stdout: PathBuf::from(m.get("stdout").unwrap()),
            stderr: PathBuf::from(m.get("stderr").unwrap()),
            env,
        }
    }

    //subs fns
    fn check_file(file_name : &String) -> Result<File, ErrMsg> {

        let path = Path::new(file_name);
        let FileErrMsgs = FileErrMsgs::new_default();

        let file = File::open(file_name);

        if !file.is_ok() {
            return Err(ErrMsg { name: String::from("file_cant_open"), msg:FileErrMsgs.file_cant_open.replace("{}", file_name)})
        }

        if !path.exists() {
            return Err(ErrMsg { name: String::from("file_no_exist"), msg:FileErrMsgs.file_no_exist.replace("{}", file_name)});
        }

        if !path.is_file() {
            return Err(ErrMsg { name: String::from("not_regular_file"), msg:FileErrMsgs.not_regular_file.replace("{}", file_name)});
        }

        if path.extension().and_then(OsStr::to_str) != Some("sconfig") {
            return Err(ErrMsg { name: String::from("file_ext_wrong"), msg:FileErrMsgs.file_ext_wrong.replace("{}", file_name)});
        }

        let opened_file = file.unwrap(); //checked that is some, will never panik
        let metadata = opened_file.metadata();

        //if dir, size is 4096, make sure to test if file before
        if !metadata.is_ok() {
            return Err(ErrMsg { name: String::from("metadata_access_denial"), msg:FileErrMsgs.metadata_access_denial.replace("{}", file_name)})
        } else {
            let metadata = metadata.unwrap();
            let size = metadata.len(); 
            if size > 10_000 || size < 50 {
                return Err(ErrMsg { name: String::from("file_size_invalid"), msg:FileErrMsgs.file_size_invalid.replace("{}", size.to_string().as_str())})
            }
        }

        Ok(opened_file)
    }

    fn read_file(file_name : &String) -> Result<Vec<String>, ErrMsg> {
        
        let FileErrMsgs = FileErrMsgs::new_default();
        let path= Path::new(file_name);

        let lines = match read_to_string(path) {
            //expect("unable to proform file to string for parser");
            Ok(content) => content,
            Err(_) => return Err(ErrMsg { name: String::from("file_extract_fail"), 
                                                msg:FileErrMsgs.file_extract_fail.replace("{}", "")})
        };

        let lines : Vec<_> = lines.split("\n").map(|e| String::from(e)).collect();
        Ok(lines)
    }

    /*fn str_add(s1 : &str, s2: &str) -> String {
        format!("{} AND {}", &s1, &s2)
    }*/

    /*fn duplicate_pgrm_name(v: &Vec<ConfigParser>) {
        let refs_to_field_pgrm_name : Vec<&String> = Vec::with_capacity(v.len()); 
        for e in v {
            refs_to_field_pgrm_name.push(&e.pgrm_name);
        }
    }*/

    pub fn main_parser(lines : Vec<String>) -> Result<Vec<ConfigParser>, ErrMsg> {
        
        let block_size = 16;
        let nb_lines = lines.len();
        if nb_lines == 0 || nb_lines % block_size != 0 { //warning 0 % 16 == 0 which means empty file     already checkedin Self::check_file
            return Err(ParserErrsMsgs::new("uneven_nb_lines", 
                                        &format!("nb_lines : {}", nb_lines.to_string())[..]));
        } 

        let mut rs : Vec<ConfigParser> = Vec::new();
        let mut pgrm_name_refs : Vec<String> = vec![];//can have two with same name
        let mut start = 0;
        let block_size= 16;

        while start < lines.len() {
            let slice = lines[start..start + block_size].to_vec();
            let r = Self::parse_file(slice);
            let r = match r{
                Err(e) => return Err(e),
                Ok(s) => s,//rs.push(s), CAUSES BORROW double borrow error on next line
            };
            rs.push(r.clone());
            //let last = (rs.len()) - 1;
            //let last : &ConfigParser = &mut rs[last]; //rs.clone().last_mut().unwrap(); //&mut rs[(&mut rs.len()) - 1];//rs.last().unwrap(); only works with immutable rs
            if pgrm_name_refs.contains(&&r.pgrm_name) {
                return Err(ParserErrsMsgs::new("prgm_name_exists", &r.pgrm_name.clone()));
            }
            pgrm_name_refs.push(r.pgrm_name.clone());
            start += block_size;
        }
        Ok(rs)
    }
    
    fn parse_file (lines : Vec<String>) -> Result<ConfigParser, ErrMsg> {
    
        let block_size = 16;
        let nb_lines = lines.len();
        if nb_lines == 0 || nb_lines % block_size != 0 { //warning 0 % 16 == 0 which means empty file     already checkedin Self::check_file
            return Err(ParserErrsMsgs::new("uneven_nb_lines", 
                                        &format!("nb_lines : {}", nb_lines.to_string())[..]));
        } 

        let mut parsed = HashMap::new(); //will be used to become ConfigParser
        let limits = ConfigParser::new_limits(); //globals
        let mut offset = 0; 

        for line in lines.iter().enumerate() {

            let FileErrMsgs = FileErrMsgs::new_default();
            let (line_nb, line) = line;
            let line = String::from(line);
            let (key, val) = Self::explode(&line, String::from(": "));
            let line_detail = format!("line{} {}=>{}", line_nb, Self::get_regex(&line_nb.to_string()),line); //for errors
            

            if line_nb == offset + 0 && !Self::is_regex(r"^prgm_name: [a-zA-Z_0-9]+$", &line) {
                return Err(ParserErrsMsgs::new("parse_err" , &line_detail));
            }
            if line_nb == offset + 1 && !Self::is_regex("^cmd: ([^;]*)$", &line) {
                return Err(ParserErrsMsgs::new("parse_err" , &line_detail));
            }
            //limit: 1-10
            if line_nb == offset + 2 && !Self::is_regex(r"^numprocs: (10|[1-9])$", &line) {
                return Err(ParserErrsMsgs::new("parse_err" , &line_detail));
            }
            //limit: 777
            if line_nb == offset + 3 && !Self::is_regex(r"^umask: [0-7]{3}$", &line) {
                return Err(ParserErrsMsgs::new("parse_err" , &line_detail));
            }
            //limit: has to exist
            if line_nb == offset + 4 {
                if !Self::is_regex(Self::get_regex("workingdir"), &line) {
                    return Err(ParserErrsMsgs::new("parse_err" , &line_detail));
                }
                if !Path::new(&val).exists() {
                    return Err(ErrMsg { name: String::from("file_no_exist"), msg:FileErrMsgs.file_no_exist.replace("{}", &line_detail)});
                }
                if !Path::new(&val).is_dir() {
                    return Err(ErrMsg { name: String::from("not_dir"), msg:format!("must be a dir {}", &line_detail)});
                }
            }
            //true false
            if line_nb == offset + 5 && !Self::is_regex(r"^autostart: (true|false)$", &line) {
                return Err(ParserErrsMsgs::new("parse_err" , &line_detail));
            }
            if line_nb == offset + 6 && !Self::is_regex(r"^autorestart: (true|false|unexpected)$", &line) {
                return Err(ParserErrsMsgs::new("parse_err" , &line_detail));
            }


            if line_nb == offset + 7 {
                if !Self::is_regex(r"^exitcodes: ([0-9]+,)+$", &line) { 
                    return Err(ParserErrsMsgs::new("parse_err", &line_detail));
                }

                //regex crate look-around not supported so this don't work : r"^exitcodes: ((25[0-5]|(2[0-4]|1\d|[1-9]|)\d)(,(?!$)|$))+$"
                let parts : Vec<_> = val.split(",").collect();
                for p in parts {
                    let n = p.parse::<i32>().unwrap_or(-1);
                    if n > 254 {
                        return Err(ParserErrsMsgs::new("not_in_range_0_254", &line_detail));
                    }
                }
            }

            //limit: 999_999_999 before u32 MAX  nb ranges pain in regex
            if line_nb == offset + 8 && !Self::is_regex(r"^startretries: [0-9]{1,9}$", &line) {
                return Err(ParserErrsMsgs::new("parse_err" , &line_detail));
            }
            if line_nb == offset + 9 && !Self::is_regex(r"^starttime: [0-9]{1,9}$", &line) {
                return Err(ParserErrsMsgs::new("parse_err" , &line_detail));
            }
            if line_nb == offset + 10 {

                if !Self::is_regex(r"^stopsignal: [A-Z]+$", &line) {
                    return Err(ParserErrsMsgs::new("parse_err" , &line_detail));
                }
                if !limits.stopsignal.contains(&String::from(&val)) {
                    return Err(ParserErrsMsgs::new("not_a_signal", &line_detail));
                }
            }

            if line_nb == offset + 11 && !Self::is_regex(r"^stoptime: [0-9]{1,9}$", &line) {
                return Err(ParserErrsMsgs::new("parse_err" , &line_detail));
            }

            if line_nb == offset + 12 {
                if !Self::is_regex(r"^stdout: [a-zA-Z0-9._/]+$", &line) {
                    return Err(ParserErrsMsgs::new("parse_err" , &line_detail));
                }
                if !Path::new(&val).exists() {
                    return Err(ErrMsg { name: String::from("file_no_exist"), msg:FileErrMsgs.file_no_exist.replace("{}", &line_detail)});
                }
                if !Path::new(&val).is_file() {
                    return Err(ErrMsg { name: String::from("not_regular_file"), msg:FileErrMsgs.not_regular_file.replace("{}", &line_detail)});
                }
            }

            if line_nb == offset + 13 {
                if !Self::is_regex(r"^stderr: [a-zA-Z0-9._/]+$", &line) {
                    return Err(ParserErrsMsgs::new("parse_err" , &line_detail));
                }
                if !Path::new(&val).exists() {
                    return Err(ErrMsg { name: String::from("file_no_exist"), msg:FileErrMsgs.file_no_exist.replace("{}", &line_detail)});
                }
                if !Path::new(&val).is_file() {
                    return Err(ErrMsg { name: String::from("not_regular_file"), msg:FileErrMsgs.not_regular_file.replace("{}", &line_detail)});
                }
            }

            if line_nb == offset + 14 && !Self::is_regex("^env: ([A-Z]+[A-Z_]+=[a-zA-Z0-9_]+,)+$", &line) {
                return Err(ParserErrsMsgs::new("env_wrong_format" , &line_detail));
            }

            if line_nb == offset + 15 && line != "" {
                return Err(ParserErrsMsgs::new("no_line_jump" , &line_detail));
            }

            if line != "" {
                parsed.insert(key, val); //except empty line
            }

            if line_nb == 16 {
                offset += block_size;
            }
          }

          let res : ConfigParser = Self::hashmap_to_ConfigParser(&parsed);
          Ok(res)

          //check if same prgm names for each block
        }

}
    

    //main fn
    
    //fn parse(&self, filename : String) {
        //if self.check_file(); //you dont  need to pass opened file to read file 
            //...
        //let lines : Vec<_> = Self::read_file(file_name);
        
        //res = parse(tokens);
    //}}

struct FileErrMsgs {

    file_no_exist : String,
    filesystem_exception : String, //not used
    file_cant_open : String,
    file_size_invalid : String,
    parser_err : String, //not used
    file_ext_wrong : String,
    not_regular_file : String,
    metadata_access_denial : String,
    file_extract_fail : String,
}

/* 
struct ParserErrsMsgs {
    parse_err : String, //general
    first_line : String,
    first_param : String,
    block_sep : String, 
    nb_over_limit : String,
    cmd_not_in_parentheses : String, 
    not_path : String,
    env_wrong_format : String,
    no_line_jump : String,
}*/

#[derive(Debug)]
pub struct ErrMsg {
    name : String, 
    msg : String,
}

impl FileErrMsgs {

    fn new_default() -> Self {
       Self { 
            file_no_exist   :  String::from("return Err: file  : |{}| does not exist"),
            filesystem_exception   :  String::from("return Err: it seems to be a filesystem exception  : |{}|"),
            file_cant_open   :  String::from("return Err: couldn't open file |{}|"),
            file_size_invalid   :  String::from("return Err: file |{}| too many characters"),
            parser_err   :  String::from("return Err: parser failed at line |{}|"),
            file_ext_wrong   :  String::from("return Err: wrong extension |{}|, must be .sconfig"),
            not_regular_file   :  String::from("return Err: not regular file |{}|"),
            metadata_access_denial : String::from("return Err: couldn't extract metadata |{}|"),
            file_extract_fail: String::from("return Err: couldn't extract content from |{}|")
       }
    }

    fn get(field : &str) -> String {
        let FileErrMsgs = Self::new_default();
        match field {
            "file_no_exist" => FileErrMsgs.file_no_exist,
            "filesystem_exception" => FileErrMsgs.filesystem_exception,
            "file_cant_open" => FileErrMsgs.file_cant_open,
            "file_size_invalid" => FileErrMsgs.file_size_invalid,
            "parser_err" => FileErrMsgs.parser_err,
            "file_ext_wrong" => FileErrMsgs.file_ext_wrong,
            "not_regular_file" => FileErrMsgs.not_regular_file,
            "metadata_access_denial" => FileErrMsgs.metadata_access_denial,
            &_ => panic!("ErrMsg field {} don't exist", field),
        }
    }

    /*fn format(err_name : String, err_str : String, err_info: String) -> ErrMsg {
        ErrMsg { name: err_name, msg:format!(err_str, err_info)}
    }*/
}

//a lot cleaner than FileErrMsgs
struct ParserErrsMsgs;
impl ParserErrsMsgs {

    fn new(which : &str, msg : &str) -> ErrMsg {
        ErrMsg {name : String::from(which),  msg : Self::get(which, format!("{}", msg))}    
    }

    fn get(field : &str, msg : String) -> String {
        //String contenation : Add trait  String + &str
        let prefix = "Parser Error";
        let errmsg = match field {
            "parse_err" => "general parsing error, recheck your input file",
            //"first_line" => "1st line must be 1.pgrm_name > val",
            //"nb_over_limit" => "to avoid DoS, choose a lower nb",
            //"cmd_not_in_parantheses" => "must begin cmd: AND cmd must be between quotes",
            //"not_path" => "file must exist and be a regular file",
            "env_wrong_format" => "all env vars must be formated env: ENV_VAR=val,ENV_VAR=val",
            "no_line_jump" => "must be a return line between each prgm block",
            "uneven_nb_lines" => "foreach profile, you must put all options, end with a linejump",
            "not_in_range_0_254" => "val must in range 0 - 254",
            //"not_in_range_0_999999999" => "val must in range 0 - 999_999_999",
            "not_a_signal" => "this signal is not implemented",
            "prgm_name_exists" => "multiple prgms have the same name",
            &_ => panic!("ParserErrsMsg field {} don't exist", field),
        };
        format!("{} ! \n type {} : {} \n extra_info : {}\n", prefix, field, errmsg, msg)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::hash::Hash;
    use std::os::unix::fs::PermissionsExt;
    use std::fs::{Permissions, set_permissions};

    fn vec_eq(v1: &Vec<String>, v2: &Vec<String>)-> bool {
        
        let mut s1 = v1.clone();
        let s1: HashSet<_> = s1.drain(..).collect();
        let mut s2 = v2.clone();
        let s2: HashSet<_> = s2.drain(..).collect();
        
        let diff: Vec<_> = s1.difference(&s2).collect();
        diff.len() == 0
    }

    fn assert_cmp_err(e1 : &ErrMsg, e2 : &ErrMsg) {
        //if beginning of err msg matches
        let mut s1 = e1.msg.split("|");
        let mut s2 = e2.msg.split("|");

        assert_eq!(s1.next(), s2.next(), "1st part of errmsg don't match \n<{}> !~= \n<{}>", e1.msg,e2.msg);
        assert_eq!(e1.name, e2.name);
    }

    fn assert_check_wrong_file(errmsg_field: &str, file_name: &String) {
        let expected_errmsg = ErrMsg {
            name : errmsg_field.to_owned(),
            msg : FileErrMsgs::get(errmsg_field)
        };

        let r = ConfigParser::check_file(file_name);
        match r {
            Ok(_) => assert!(false, "was suppose to return Err(errmsg)"),
            Err(errmsg) => assert_cmp_err(&expected_errmsg, &errmsg),
        };
    }
    
    fn assert_check_right_file(file_name: &String) {
        let r = ConfigParser::check_file(file_name);
        assert!(r.is_ok());
    }

    //for test check file
    fn set_file_permission (file_name : &String, perm : &str) {
        //in order to push 
        let path = Path::new(file_name);

        let write_only_flag = 0o6200; //-wS--S---
        let all_flag = 0o0777; //rwxrwxrwx

        match perm {
            "write_only" => set_permissions(path, Permissions::from_mode(write_only_flag)).   
                                                                    expect("failed to set file to write_only"),
            //we need default case to exhaust match, this arm is to set file perm back to a read flag when the tests are over  in order to push on github
            &_ => set_permissions(path, Permissions::from_mode(all_flag)).
                                                                    expect("end of cargo  test : failed to set file  permissionsback to 0o777"),
        }

    }

  

    #[test]
    fn test_fn__check_file() {

        let test_dir = Path::new("./test_docs/file_state");
        assert!(test_dir.exists(), "couldnt access test dir {}", test_dir.to_str().unwrap());

        let wrong_format1 = test_dir.join("wrong.wrong").to_str().unwrap().to_owned();
        let wrong_format2 = test_dir.join("wrong_file_name").to_str().unwrap().to_owned();
        let wrong_format3 = test_dir.join(".wrong_file_name").to_str().unwrap().to_owned();
        let too_big = test_dir.join("too_many_chars.sconfig").to_str().unwrap().to_owned();
        let is_a_dir = test_dir.join("not_file.sconfig").to_str().unwrap().to_owned();

        //rust test has no teardown, if a panic occurs, there's no way to reset file perm to normal so let's keep all as close otgether a possible
        //we have to reset to no normal  by giving read  access so we can push on github
        let unaccessible = test_dir.join("unaccessible.sconfig").to_str().unwrap().to_owned();
        set_file_permission(&unaccessible, "write_only");
        assert_check_wrong_file("file_cant_open", &unaccessible);
        set_file_permission(&unaccessible, "all");

        assert_check_wrong_file("not_regular_file", &is_a_dir);
        assert_check_wrong_file("file_size_invalid", &too_big);
        assert_check_wrong_file("file_ext_wrong", &wrong_format1);
        assert_check_wrong_file("file_ext_wrong", &wrong_format2);
        assert_check_wrong_file("file_ext_wrong", &wrong_format3);

        let correct1 = test_dir.join("correct.sconfig").to_str().unwrap().to_owned();
        assert_check_right_file(&correct1);


    }

    #[test]
    fn test_fn__read_file() {

        let correct_file_1 = String::from("./test_docs/parser_tests/small.txt");
        let res = ConfigParser::read_file(&correct_file_1);
        assert!(res.is_ok());
        let res_content = res.unwrap();
        let expected_content : Vec<String> = vec![
            String::from("Lorem Ipsum is simply dummy text of the printing and typesetting industry."),
            String::from("Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, "),
            String::from("when an unknown printer took a galley of type and scrambled it to make a type specimen book."),
            String::from(""),
        ];

        assert!(vec_eq(&res_content, &expected_content), "res content {:?}", res_content);

        let not_existing = String::from("");
        let res = ConfigParser::read_file(&not_existing);
        assert!(res.is_err());
        match res {
            Ok(_) => (), //will never happen
            Err(r) =>  assert_eq!(r.name, "file_extract_fail"),
        }    
   }


   fn assert_parser(correct_content: &Vec<String>, field: &str, val:&str,
                     expected_err_name : &str, state: &str) {
        let mut content = correct_content.clone();

        let field_nb = match field {
            "line_jump" => 15,
            _ => content.iter().position(|r| r.contains(field)).expect(&format!("can't find index of {} in Vec<String>, you wrote the field wrong", field)),
        };
        //let field_nb = strvec_find(&content, &String::from(field));
        content[field_nb] = format!("{}: {}",field, val);
        let res = ConfigParser::parse_file(content);
        if state == "fail" {
            assert!(res.is_err(), "ASSERT failed for FIELD:{} VAL:{} EXPECTED_ERR_NAME:{} STATE:{}", field, val, expected_err_name,state);
            match res {
                Ok(r) => (),  //will never happen
                Err(r) => assert_eq!(r.name, *expected_err_name),
            };  
        }
    }

   #[test]
   fn test_fn__parse_file() {
        /*
            TESTS TO BE PROFORMED IN ORDER
            valid : global correct 


            invalid : under nb limit on all vals
            invalid : over nb limit on all vals 
            invalid : 

            invalid : for each field test under/over/unexisted/ limit vals
            
        */



        //from ConfigParser::read_file
        //this vec is used(cloned) by all error tests
        let correct_content1 = vec![
            String::from("prgm_name: nginx"),
            String::from(r#"cmd: /usr/local/bin/nginx -c "/etc/nginx/test.conf" -x"#),
            String::from("numprocs: 1"),
            String::from("umask: 022"),
            String::from("workingdir: /tmp"),
            String::from("autostart: true"),
            String::from("autorestart: unexpected"),
            String::from("exitcodes: 0,2,"),
            String::from("startretries: 3"),
            String::from("starttime: 5"),
            String::from("stopsignal: SIGTERM"),
            String::from("stoptime: 10"),
            String::from("stdout: ./test_docs/parser_tests/file.stdout"),
            String::from("stderr: ./test_docs/parser_tests/file.stderr"),
            String::from("env: STARTED_BY=taskmaster,ANSWER=42,"),
            String::from(""),
        ];

        let expected_res : ConfigParser = ConfigParser {
            pgrm_name: String::from("nginx"),
            cmd: vec!["/usr/local/bin/nginx".to_string(), "-c".to_string(), String::from("/etc/nginx/test.conf"), "-x".to_string()],
            numprocs: 1 as u32,
            umask: 22 as u32,
            workingdir: PathBuf::from("/tmp"),
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

        let res = ConfigParser::parse_file(correct_content1.clone());
        match &res {
            Ok(r) => (),
            Err(r) => assert!(false, "{} {}", r.name, r.msg),
        };
        let res= res.unwrap();

        assert_eq!(expected_res, res);

        let res = ConfigParser::main_parser(correct_content1.clone());
        match &res {
            Ok(r) => (),
            Err(r) => assert!(false, "{} {}", r.name, r.msg),
        };
        let multi_res= res.unwrap();
        assert_eq!(multi_res, vec![expected_res]);


        //from ConfigParser::read_file
        let correct_content_2_prgms = vec![
            String::from("prgm_name: LS"),
            String::from(r#"cmd: ls -la "text" -x"#),
            String::from("numprocs: 10"),
            String::from("umask: 777"),
            String::from("workingdir: ./"),
            String::from("autostart: false"),
            String::from("autorestart: true"),
            String::from("exitcodes: 254,"),
            String::from("startretries: 999999999"),
            String::from("starttime: 999999999"),
            String::from("stopsignal: SIGKILL"),
            String::from("stoptime: 999999999"),
            String::from("stdout: ./test_docs/parser_tests/file.stdout"),
            String::from("stderr: ./test_docs/parser_tests/file.stderr"),
            String::from("env: V_=0,"),
            String::from(""),
            String::from("prgm_name: LS2"),
            String::from(r#"cmd: ls -la "text" -x"#),
            String::from("numprocs: 10"),
            String::from("umask: 777"),
            String::from("workingdir: ./"),
            String::from("autostart: false"),
            String::from("autorestart: true"),
            String::from("exitcodes: 254,"),
            String::from("startretries: 999999999"),
            String::from("starttime: 999999999"),
            String::from("stopsignal: SIGKILL"),
            String::from("stoptime: 999999999"),
            String::from("stdout: ./test_docs/parser_tests/file.stdout"),
            String::from("stderr: ./test_docs/parser_tests/file.stderr"),
            String::from("env: V_=0,"),
            String::from(""),
        ];

        let expected_res_2_prgms : Vec<ConfigParser> = vec![ConfigParser {
            pgrm_name: String::from("LS"),
            cmd: vec!["ls".to_string(), "-la".to_string(), String::from("text"), "-x".to_string()],
            numprocs: 10 as u32,
            umask: 777 as u32,
            workingdir: PathBuf::from("./"),
            autostart: false,
            autorestart: String::from("true"),
            exitcodes: vec![254 as u32],
            startretries: 999_999_999 as u32,
            starttime: 999_999_999 as u32,
            stopsignal: String::from("SIGKILL"),
            stoptime: 999_999_999 as u32,
            stdout: PathBuf::from("./test_docs/parser_tests/file.stdout"),
            stderr: PathBuf::from("./test_docs/parser_tests/file.stderr"),
            env: HashMap::from([
                (String::from("V_"), String::from("0")),
            ])
        },
        ConfigParser {
            pgrm_name: String::from("LS2"),
            cmd: vec!["ls".to_string(), "-la".to_string(), String::from("text"), "-x".to_string()],
            numprocs: 10 as u32,
            umask: 777 as u32,
            workingdir: PathBuf::from("./"),
            autostart: false,
            autorestart: String::from("true"),
            exitcodes: vec![254 as u32],
            startretries: 999_999_999 as u32,
            starttime: 999_999_999 as u32,
            stopsignal: String::from("SIGKILL"),
            stoptime: 999_999_999 as u32,
            stdout: PathBuf::from("./test_docs/parser_tests/file.stdout"),
            stderr: PathBuf::from("./test_docs/parser_tests/file.stderr"),
            env: HashMap::from([
                (String::from("V_"), String::from("0")),
            ])
        }];

        let res = ConfigParser::main_parser(correct_content_2_prgms.clone());
        match &res {
            Ok(r) => (),
            Err(r) => assert!(false, "{} {}", r.name, r.msg),
        };
        let multi_res= res.unwrap();
        assert_eq!(multi_res,  expected_res_2_prgms.clone());

        /////////////////ERROR TESTS 
        //let expected_res_copy = expected_res.clone();
        let mut correct_content_2_prgms_c = correct_content_2_prgms.clone();
        correct_content_2_prgms_c[16] = correct_content_2_prgms_c[0].clone();
        let res = ConfigParser::main_parser(correct_content_2_prgms_c);
        match &res {
            Ok(r) => assert!(false, "suppose to fail with same prgm name"),
            Err(r) => assert_eq!(r.name,"prgm_name_exists"),
        };

        
        

        //TEST: invalid field prgm_name
        //let invalid_prgm_name_1 = String::from(":");
        //let invalid_prgm_name_2 = ;
        //let invalid_prgm_name_3 = ;


        //TEST: invalid field cmd
        //let invalid_cmd_ops = r#"";

        //TEST: invalid field numprocs
        let invalid_under_numprocs = "0";
        let invalid_over_numprocs = "11";
        assert_parser(&correct_content1, "numprocs", invalid_under_numprocs,  "parse_err", "fail");
        assert_parser(&correct_content1, "numprocs", invalid_over_numprocs,  "parse_err", "fail");
        

        //TEST: invalid field umask
        let invalid_over_umask = "778";
        assert_parser(&correct_content1, "umask", invalid_over_umask,  "parse_err", "fail");

        //TEST: invalid field exitcodes
        let invalid_exitcodes = "";
        assert_parser(&correct_content1, "exitcodes", invalid_exitcodes,  "parse_err", "fail");
        let invalid_exitcodes = "255";
        assert_parser(&correct_content1, "exitcodes", invalid_exitcodes,  "parse_err", "fail");
        let invalid_exitcodes = "0"; //no comma
        assert_parser(&correct_content1, "exitcodes", invalid_exitcodes,  "parse_err", "fail");
        let invalid_exitcodes = "-1";
        assert_parser(&correct_content1, "exitcodes", invalid_exitcodes,  "parse_err", "fail");
        
        //TEST: invalid field startretries
        //TEST: invalid field starttime
        //TEST: invalid field stoptime
        let invalid_over_999999999 = "1000000000";
        assert_parser(&correct_content1, "startretries", invalid_over_999999999,  "parse_err", "fail");
        assert_parser(&correct_content1, "starttime", invalid_over_999999999,  "parse_err", "fail");
        assert_parser(&correct_content1, "stoptime", invalid_over_999999999,  "parse_err", "fail");


        //TEST: invalid field stopsignal
        let unknown_signal = "SIGCPP";
        assert_parser(&correct_content1, "stopsignal", unknown_signal,  "not_a_signal", "fail");

        //TEST: invalid field stdout
        //TEST: invalid field stderr
        let not_a_file = "./test_docs/parser_tests";
        assert_parser(&correct_content1, "stdout", not_a_file,  "not_regular_file", "fail");
        let unexisting_file = "./xyz";        
        assert_parser(&correct_content1, "stdout", unexisting_file,  "file_no_exist", "fail");

        //TEST: invalid field workingdir
        let not_a_dir = "./test_docs/parser_tests/file.stdout";
        assert_parser(&correct_content1, "workingdir", not_a_dir,  "not_dir", "fail");
        let unexisting_file = "./xyz";        
        assert_parser(&correct_content1, "workingdir", unexisting_file,  "file_no_exist", "fail");

        //TEST: invalid field env
        let anything = "2";
        assert_parser(&correct_content1, "env", anything,  "env_wrong_format", "fail");
        let small_chars_key = "key=val,";
        assert_parser(&correct_content1, "env", small_chars_key,  "env_wrong_format", "fail");
        let no_comma =  "START=true";
        assert_parser(&correct_content1, "env", no_comma,  "env_wrong_format", "fail");
        let no_comma2 =  "START=true,END=false";
        assert_parser(&correct_content1, "env", no_comma2,  "env_wrong_format", "fail");
        let start_with_underscore =  "_START=true,";
        assert_parser(&correct_content1, "env", start_with_underscore,  "env_wrong_format", "fail");
        let second_start_with_underscore =  "ME=correct,_HERE=wrong,";
        assert_parser(&correct_content1, "env", second_start_with_underscore,  "env_wrong_format", "fail");

    

        //TEST: invalid NO line jump
        let replace_line_jump = "-";
        assert_parser(&correct_content1, "line_jump", replace_line_jump,  "no_line_jump", "fail");




        
    }

        

}




/* 
#[derive(Debug)]
struct X{
    n:u32,
    id:u32,
    code:u32,
}


fn main() {

let v1 = vec![1,2,3];
let x1 = X{n:1,id:2,code:3};
println!("{:?} \n {:?}\n", v1, x1);

let mut v2 = v1.clone();
v2[0] = 9;
let x2 = X{code:777, ..x1};
println!("{:?} \n {:?}\n", v2, x2);

}






*/
