
use std::collections::HashMap;
/// Create &a new ThreadPool.
///
/// The size is the number of threads in the pool.
///
/// # Panics
///
/// The `new` function will panic if the size is zero.


use std::fs::{File, read_to_string};
use std::path::Path;
use std::ffi::OsStr;
use regex::Regex;


struct ConfigParser {
    pgrm_name : String,
    cmd : String,
    numprocs  : u8,
    umask  :  u8,
    workingdir  : Path,
    autostart  : bool,
    autorestart  : bool,
    exitcodes  : Vec<u8>,
    startretries  :  u8,
    starttime  : u8,
    stopsignal  : String,
    stoptime  :  u8,
    stdout  :  Path,
    stderr  :  Path,
    env  : HashMap<String, String>,
}

struct Limits {
    //cmp with ConfigParser inst.
    nb_prgms : Regex, //anti DoS atk
    pgrm_name : Regex,
    cmd : Regex, //anything between  
    numprocs  : u8, //anti DoS atk
    umask  : u8,
    exitcodes  : u8,
    startretries  : u8,
    starttime  : u8,
    stopsignal  : [String; 13],
    stoptime  : u8,
    env  : Regex,
}

impl ConfigParser {

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
            nb_prgms : 10, //anti DoS atk
            pgrm_name : Regex::new(r"^pgrm_name > [a-zA-Z_]+$").unwrap(),
            cmd : Regex::new(r#""([^;]*)""#).unwrap(), //anything between " "
            numprocs  : 10, //anti DoS atk
            umask  : 0o777,
            exitcodes  : u8::MAX,
            startretries  : u16::MAX,
            starttime  : u16::MAX,
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
            stoptime  : 10,
            env  : Regex::new(r"^[a-zA-Z_]+=[a-zA-Z0-9 ]+$").unwrap()
        }
    }

    //utility fns
    fn is_regex(reg : &str , s : &str) -> bool {
        let re = Regex::new(reg).unwrap();
        re.is_match(s)
    }

    fn explode(s : &'a str , token : &'a str) -> (&str, &str) {
        let g : Vec<_> = s.split(token).collect();
        (g.get(0) , g.get(1))
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

    fn read_file(file_name : &String) -> Result<Vec<&str>, ErrMsg> {
        
        let FileErrMsgs = FileErrMsgs::new_default();
        let path= Path::new(file_name);

        let lines = match read_to_string(path) {
            //expect("unable to proform file to string for parser");
            Ok(content) => content,
            Err(e) => return Err(ErrMsg { name: String::from("file_extract_fail"), 
                                                msg:FileErrMsgs.file_extract_fail.replace("{}", e.to_str())})
        };

        let lines : Vec<_> = lines.split("\n").collect();
        Ok(lines)
    }


    
    fn parse_file (lines : Vec<&str>) -> Result<ConfigParser, ErrMsg> {

        //two prgms cant have the same name 
        //parser AND CHECK if lines valid
        //each option is in once per pgrm 
        //umask is a umask , stdout is a path, numprocs is a n int not neg nor over 1000 DoS attack
    
        //global tests
        //we checked file size in check_file
        let block_size = 14;
        let nb_lines = lines.len();
        if nb_lines == 0 || nb_lines % block_size != 0 { //warning 0 % 14 == 0 which means empty file     already checkedin Self::check_file
            return ParserErrsMsgs::new(String::from("uneven_nb_lines") , 
                                        &format!("nb_lines : {}", nb_lines.to_string())[..])
        } 

        let mut parsed = HashMap::new(); //shouldbe config parser
        let limits = ConfigParser::new_limits();
        let mut offset = 0; 

        for line in lines.iter().enumerate() {

            let (line_nb, line) = line;
            let (key, val) = Self::explode(line, ": ");
            let line_detail = format!("line {} : {}", line_nb, line).to_str(); //for errors
            

            if line_nb == offset + 0 && Self::is_regex(r"^pgrm_name: (a-zA-Z_]+)$", line) {
                parsed.insert(key, val);
            }
            if line_nb == offset + 1 && Self::is_regex(r#"^cmd: "([^;]*)"$"#, line) {
                parsed.insert(key, val);
            }
            //limit: 1-10
            if line_nb == offset + 2 && Self::is_regex(r"^numprocs: (10|[1-9])$", line) {
                parsed.insert(key, val);
            }
            //limit: 777
            if line_nb == offset + 3 && Self::is_regex(r"^umask: [0-7]{3})$", line) {
                parsed.insert(key, val);
            }
            //limit: has to exist
            if line_nb == offset + 4 && Self::is_regex(r"^workingdir: [a-zA-Z0-9_/]+$", line) && Path::new(line).exists() {
                parsed.insert(key, val);
            }
            //true false
            if line_nb == offset + 5 && Self::is_regex(r"^autostart: (true|false)$", line) {
                parsed.insert(key, val);
            }
            if line_nb == offset + 6 && Self::is_regex(r"^autorestart: (true|false)$", line) {
                parsed.insert(key, val);
            }


            if line_nb == offset + 7 && line.starts_with("exitcodes: "){
                //regex crate look-around not supported so this don't work : r"^exitcodes: ((25[0-5]|(2[0-4]|1\d|[1-9]|)\d)(,(?!$)|$))+$"
                let parts : Vec<_> = val.split(",").collect();
                for p in parts {
                    let n = p.parse::<i32>().unwrap_or(-1);
                    if n < 0 || n > 254 {
                        return ParserErrsMsgs::new(String::from("not_in_range_0_254") , line_detail);
                    }
                }
            } else {
                parsed.insert(key, val);
            }

            //limit: 999_999_999 before u32 MAX  nb ranges pain in regex
            if line_nb == offset + 8 && Self::is_regex(r"^startretries: [0-9]{1..9}$", line) {
                parsed.insert(key, val);
            }
            if line_nb == offset + 9 && Self::is_regex(r"^starttime: [0-9]{1..9}$", line) {
                parsed.insert(key, val);
            }
            if line_nb == offset + 10 && Self::is_regex(r"^stopsignal: [A-Z]+$", line) {
                if !limits.stopsignal.contains(val) {
                    return ParserErrsMsgs::new(String::from("not_a_signal") , line_detail);
                }
            } else {
                parsed.insert(key, val);
            }

            if line_nb == offset + 11 && Self::is_regex(r"^stoptime: [0-9]{1..9}$", line) {
                parsed.insert(key, val);
            }

            if line_nb == offset + 12 && Self::is_regex(r"^stdout: [a-zA-Z0-9_/]+$", line) && Path::new(line).exists() {
                parsed.insert(key, val);
            }

            if line_nb == offset + 13 && Self::is_regex(r"^stderr: [a-zA-Z0-9_/]+$", line) && Path::new(line).exists() {
                parsed.insert(key, val);
            }

            if line_nb == offset + 14 && Self::is_regex("^env: ([A-Z]+[A-Z_]+=[a-zA-Z0-9]+,)+$", line) {
                parsed.insert(key, val);
            }

            if line_nb == offset + 14 && line != "" {
                return ParserErrsMsgs::new(String::from("no_line_jump") , line_detail);
            }


            if error {
                return ParserErrMsgs::new(key , val)
            }
      
            offset += block_size;
          }

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

struct ErrMsg {
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

    fn new(which : String , msg : &str) -> ErrMsg {
        ErrMsg {name : which,  msg : Self::get(which, format!("{}", msg).to_str())}    
    }

    fn get(field : &str, msg : String) -> String {
        //String contenation : Add trait  String + &str
        let prefix = "Parser Error";
        let errmsg = match field {
            "parse_err" => "general parsing",
            "first_line" => "1st line must be 1.pgrm_name > val",
            "nb_over_limit" => "to avoid DoS, choose a lower nb",
            "cmd_not_in_parantheses" => "cmd must be between quotes",
            "not_path" => "file must exist and be a regular file",
            "env_wrong_format" => "all env vars must be formated ENV_VAR=val",
            "no_line_jump" => "must be a return line between each prgm block",
            "uneven_nb_lines" => "foreach profile, you must put all options, end with a linejump",
            "not_in_range_0_254" => "val must in range 0 - 254",
            "not_a_signal" => "this signal is not implemented",
            &_ => panic!("ParserErrsMsg field {} don't exist", field),
        };
        format!("{} > {} : {} |{}|", prefix, field, errmsg, msg)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;
    use std::fs::{Permissions, set_permissions};

    fn assert_cmp_err(e1 : &ErrMsg, e2 : &ErrMsg) {
        //if beginning of err msg matches
        let mut s1 = e1.msg.split("|");
        let mut s2 = e2.msg.split("|");

        assert_eq!(s1.next(), s2.next(), "1st part of errmsg don't match \n<{}> !~= \n<{}>", e1.msg,e2.msg);
        assert_eq!(e1.name, e2.name);
    }

    fn assert_check_file(errmsg_field: &str, file_name: &String) {
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
        assert_check_file("file_cant_open", &unaccessible);
        set_file_permission(&unaccessible, "all");

        assert_check_file("not_regular_file", &is_a_dir);
        assert_check_file("file_size_invalid", &too_big);
        assert_check_file("file_ext_wrong", &wrong_format1);
        assert_check_file("file_ext_wrong", &wrong_format2);
        assert_check_file("file_ext_wrong", &wrong_format3);
    }

    fn test_fn__parse() {

        
    }


}
