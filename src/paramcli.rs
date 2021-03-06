use std::env;
use std::fs::File;

#[derive(Debug)]
pub struct Paramcli
//parameters from command line and/or confFile
{
    pub search: String,
    pub replace: String,
    pub file: String,
    pub only_first: bool,
    pub keep_old: bool,
}

impl Default for Paramcli {
    fn default() -> Self {
        Paramcli::new()
    }
}

impl Paramcli {
    pub fn new() -> Paramcli {
        let mut fic = String::new();
        let mut search = String::new();
        let mut replace = String::new();
        let mut only_first = false;
        let mut keep_old = false;
        let mut replace_param = false;

        let args: Vec<String> = env::args().skip(1).collect();
        let name = env::args()
            .take(1)
            .next()
            .unwrap_or_else(|| String::from("search_and_replace"));
        println!("{} 1.0 (2020)", name);
        if args.is_empty() {
            help(&name);
        }
        for arg in args {
            if arg == "/?"
                || arg == "-?"
                || arg.to_lowercase() == "/help"
                || arg.to_lowercase() == "-help"
            {
                help(&name);
            }
            if let Some(n) = get_param(&arg, String::from("/search:")) {
                search = n;
                continue;
            }
            if let Some(n) = get_param(&arg, String::from("/replace:")) {
                replace = n;
                replace_param = true;
                continue;
            }
            if let Some(n) = get_param(&arg, String::from("/fic:")) {
                fic = n;
                continue;
            }
            if get_param(&arg, String::from("/only_first")).is_some() {
                only_first = true;
                continue;
            }
            if get_param(&arg, String::from("/keep_old")).is_some() {
                keep_old = true;
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
        if !replace_param {
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
            only_first,
            keep_old,
        }
    }
}

fn get_param(arg: &str, switch: String) -> Option<String> {
    if arg.to_lowercase().starts_with(&switch) {
        let mut a = String::from(arg);
        return Some(a.split_off(switch.len()));
    }
    None
}

fn help(name: &str) {
    println!("syntax : {} /search:search_string /replace:replace_string /fic:file [/only_first] [/keep_old]",name);
    println!("paramerters between [] are optionnals");
    println!("------------------------------------");
    println!("search_string: String to find and to replace");
    println!("replace_string: String to put in place of search_string");
    println!("fic: file where to search");
    println!("/only_first: if search_string if found many times, only the first one is replaced");
    println!("/keep_old: do a .old copy of original file");
    std::process::exit(0);
}
