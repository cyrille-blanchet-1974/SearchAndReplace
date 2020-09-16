use std::env;
use std::fs::File;

#[derive(Debug)]
pub struct Paramcli
//parameters from command line and/or confFile
{
    pub search: String,
    pub replace: String,
    pub file: String,
    pub only_first: bool
}

impl Paramcli {
    pub fn new() -> Paramcli {
        let mut fic = String::new();
        let mut search = String::new();
        let mut replace = String::new();
        let mut only_first = false;

        let args: Vec<String> = env::args().skip(1).collect();
        let name = env::args().take(1).next().unwrap_or_else(||String::from("search_and_replace"));
        if args.is_empty() {
            help(&name);
        }
        for arg in args {
            println!("{}", arg);
            if arg == "/?"
                || arg == "-?"
                || arg.to_lowercase() == "/help"
                || arg.to_lowercase() == "-help"
            {
                help(&name);
            }
            if arg.to_lowercase().starts_with("/search:") {
                search = get_param(arg);
                continue;
            }
            if arg.to_lowercase().starts_with("/replace:") {
                replace = get_param(arg);
                continue;
            }
            if arg.to_lowercase().starts_with("/fic:") {
                fic = get_param(arg);
                continue;
            }

            if arg.to_lowercase() == "/only_first" {
                only_first = true;
                continue;
            }
        }
        //checks
        if fic.is_empty() {
            println!("ERROR! no file to work with!");
            println!("--------------------------------------------------");
            help(&name);
        }
        if search.is_empty() {
            println!("ERROR! nothing to search!");
            println!("--------------------------------------------------");
            help(&name);
        }
        if replace.is_empty() {
            println!("ERROR! nothing to replace!");
            println!("--------------------------------------------------");
            help(&name);
        }        
        if search == replace {
            println!("ERROR! search and replace strings are equals!");
            println!("--------------------------------------------------");
            help(&name);
        }
        //check if file exists
        if File::open(&fic).is_err() {
            println!("Error file {} doesn't exists or unereadable", &fic);
            help(&name);
        };
        Paramcli {
            search,
            replace,
            file: fic,
            only_first
        }
    }
}

fn get_param(arg: String) -> String {
    let mut res = String::new();
    for part in arg.split(':').skip(1) {
        if !res.is_empty() {
            res.push_str(":");
        }
        res.push_str(part);
    }
    if arg.ends_with(':') {
        res.push_str(":");
    }
    res
}

fn help(name:&str) {
    println!("syntax : {} /search:search_string /replace:replace_string /fic:file [/only_first]",name);
    println!("paramerters between [] are optionnals");
    println!("------------------------------------");
    println!("search_string: String to find and to replace");
    println!("replace_string: String to put in place of searchçstring");
    println!("fic: file where to search");
    println!("/only_first: if search_string if found many times, only the first one is replaced");
    std::process::exit(0);
}
