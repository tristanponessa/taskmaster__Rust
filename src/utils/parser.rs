
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
    stopsignal  : [&str; 5],
    stoptime  : u8,
    env  : Regex,
}

impl ConfigParser {

    fn new_default() -> Self {

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
    }

    fn new_limits() -> Limits {
        Limits {
            nb_prgms : 10, //anti DoS atk
            pgrm_name : Regex::new(r"^[a-zA-Z_]+$").unwrap(),
            cmd : Regex::new(r#""([^;]*)""#).unwrap(), //anything between " "
            numprocs  : 10, //anti DoS atk
            umask  : 777,
            exitcodes  : u8::MAX,
            startretries  : u8::MAX,
            starttime  : u8::MAX,
            stopsignal  : ["SIGKILL", "SIGTERM", "SIGBUS", "SIGABORT", "HANGUP"],//array of all 16 signals morebut 16 is used for some reason 
            stoptime  : 10,
            env  : Regex::new(r"^[a-zA-Z_]+$").unwrap()
        }
    }

    //subs fns

    fn check_file(file_name : &String) -> Result<File, ErrMsg> {

        let path = Path::new(file_name);
        let errmsgs = ErrMsgs::new_default();

        let file = File::open(file_name);

        if !file.is_ok() {
            return Err(ErrMsg { name: String::from("file_cant_open"), msg:errmsgs.file_cant_open.replace("{}", file_name)})
        }

        if !path.exists() {
            return Err(ErrMsg { name: String::from("file_no_exist"), msg:errmsgs.file_no_exist.replace("{}", file_name)});
        }

        if !path.is_file() {
            return Err(ErrMsg { name: String::from("not_regular_file"), msg:errmsgs.not_regular_file.replace("{}", file_name)});
        }

        if path.extension().and_then(OsStr::to_str) != Some("sconfig") {
            return Err(ErrMsg { name: String::from("file_ext_wrong"), msg:errmsgs.file_ext_wrong.replace("{}", file_name)});
        }

        let opened_file = file.unwrap(); //checked that is some, will never panik
        let metadata = opened_file.metadata();

        if !metadata.is_ok() {
            return Err(ErrMsg { name: String::from("metadata_access_denial"), msg:errmsgs.metadata_access_denial.replace("{}", file_name)})
        } else {
            let metadata = metadata.unwrap();
            let size = metadata.len(); //if dir, size is 4096
            if size > 500 {
                return Err(ErrMsg { name: String::from("file_too_big"), msg:errmsgs.file_too_big.replace("{}", size.to_string().as_str())})
            }
        }

        
        
        Ok(opened_file)
    }

    fn parse_file (file_name : &String) -> Result<ConfigParser, ErrMsg> {

        //two prgms cant have the same name 
        //parser AND CHECK if lines valid
        //each option is in once per pgrm 
        //umask is a umask , stdout is a path, numprocs is a n int not neg nor over 1000 DoS attack
        


        let errmsgs = ErrMsgs::new_default();
        let path= Path::new(file_name);

        //read_file
        let lines = match read_to_string(path) {
            //expect("unable to proform file to string for parser");
            Ok(r) => r,
            Err(e) => return Err(ErrMsg { name: String::from("file_extract_fail"), msg:errmsgs.file_extract_fail.replace("{}", file_name)})
        };

        let lines : Vec<_> = lines.split("\n").collect();
        let mut parsed = HashMap::new();

        let limits

        for line in lines.iter().enumerate() {

            let (line_nb, line) = line;
            //1st line
            if line_nb == 0 && line.contains("pgrm_name")
            //get nb of line for errs
            /*1st line 1st option pgrm_name */
            /* */
            let r = line.split(":").collect();



            
            
            //cmd line option
            if line.contains(r#"""#) {
                let r = l.split(r#"""#).collect();

                let key = 
                let cmd = r.
            } else {
                
            }*/

            
        }





    }
    

    //main fn
    /*
    fn parse(&self, filename : String) {
        //opened_file = self.check_file();
        //tokens = read_file(open_file);
        //res = parse(tokens);
    }*/
}

struct ErrMsgs {

    file_no_exist : String,
    filesystem_exception : String,
    file_cant_open : String,
    file_too_big : String,
    parser_err : String,
    file_ext_wrong : String,
    not_regular_file : String,
    metadata_access_denial : String,
    file_extract_fail : String,
}

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
}

struct ErrMsg {
    name : String, 
    msg : String,
}

impl ErrMsgs {
    fn new_default() -> Self {
       Self { 
            file_no_exist   :  String::from("return Err: file  : |{}| does not exist"),
            filesystem_exception   :  String::from("return Err: it seems to be a filesystem exception  : |{}|"),
            file_cant_open   :  String::from("return Err: couldn't open file |{}|"),
            file_too_big   :  String::from("return Err: file |{}| too many characters"),
            parser_err   :  String::from("return Err: parser failed at line |{}|"),
            file_ext_wrong   :  String::from("return Err: wrong extension |{}|, must be .sconfig"),
            not_regular_file   :  String::from("return Err: not regular file |{}|"),
            metadata_access_denial : String::from("return Err: couldn't extract metadata |{}|"),
            file_extract_fail: String::from("return Err: couldn't extract content from |{}|")
       }
    }

    fn get(field : &str) -> String {
        let errmsgs = Self::new_default();
        match field {
            "file_no_exist" => errmsgs.file_no_exist,
            "filesystem_exception" => errmsgs.filesystem_exception,
            "file_cant_open" => errmsgs.file_cant_open,
            "file_too_big" => errmsgs.file_too_big,
            "parser_err" => errmsgs.parser_err,
            "file_ext_wrong" => errmsgs.file_ext_wrong,
            "not_regular_file" => errmsgs.not_regular_file,
            "metadata_access_denial" => errmsgs.metadata_access_denial,
            &_ => panic!("ErrMsg field {} don't exist", field),
        }
    }

    /*fn format(err_name : String, err_str : String, err_info: String) -> ErrMsg {
        ErrMsg { name: err_name, msg:format!(err_str, err_info)}
    }*/
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
            msg : ErrMsgs::get(errmsg_field)
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
    fn regex_test_env_vars() {
        
        use regex::Regex;

    //valid
    let str1 = "STARTED_BY=taskmaster;";
    let str2 = "STARTEDBY=taskmaster;";
    let str3 = "_STARTED_BY=taskmaster;ANSWE_R=42;";
    let str7 = "STARTED_BY=taskmaster;STARTED_BY=44;";
    
    
    //fails
    let str12 = "STARTED_BY=taskmaster;STARTED_BY=44";
    let str10 = "STARTED_BY=taskmaster STARTED_BY=44";
    let str11 = "STARTED_BY=taskmaster STARTED_BY=44;";
    let str4 = "STARTED_BY=taskmaster";
    let str5 = "_=taskmaster";
    let str6 = "=taskmaster";
    let str8 = "STARTED_BY=;";
    let str9 = "STARTED_BY= ;";
    
    

    let re = Regex::new(r"([A-Z_]+=[a-zA-Z_0-9]+;)+").unwrap();
    
    let caps = re.captures(str11).unwrap();
    for c in caps.iter() {
        println!("{:?}", c);
    }
    
    
    assert!(re.is_match(str1)); // I expect "01" on stdout.
    assert!(re.is_match(str2)); // I expect "01" on stdout.
    assert!(re.is_match(str3)); // I expect "01" on stdout.
    assert!(re.is_match(str7)); // I expect "01" on stdout.
    
    assert_eq!(false, re.is_match(str4)); // I expect "01" on stdout.
    assert_eq!(false, re.is_match(str5)); // I expect "01" on stdout.
    assert_eq!(false, re.is_match(str6)); // I expect "01" on stdout.
    assert_eq!(false, re.is_match(str10), "{}", str10); // I expect "01" on stdout.
    assert_eq!(false, re.is_match(str11), "{}", str11); // I expect "01" on stdout.
    assert_eq!(false, re.is_match(str8)); // I expect "01" on stdout.
    assert_eq!(false, re.is_match(str9)); // I expect "01" on stdout.
    assert_eq!(false, re.is_match(str12)); // I expect "01" on stdout.
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
        assert_check_file("file_too_big", &too_big);
        assert_check_file("file_ext_wrong", &wrong_format1);
        assert_check_file("file_ext_wrong", &wrong_format2);
        assert_check_file("file_ext_wrong", &wrong_format3);
    }

    fn test_fn__parse() {

        
    }


}
