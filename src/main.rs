use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::sync::mpsc::channel;
use std::thread::{spawn, JoinHandle};
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

pub fn start_thread_read(
    to_search: Sender<String>,
    fic: &str,
) -> JoinHandle<()> {
    let file = String::from(fic);
    spawn(move || {
        let input = File::open(&file);
        match input {
            Err(e) => {
                println!("Error reading file {} => {}", &file, e);
            }
            Ok(f) => {
                let buffered = BufReader::new(f);
                for line in buffered.lines() {
                    if let Ok(l) = line {
                        if to_search.send(l).is_err() {
                            println!("error sending to search");
                            return;
                        }
                    } 
                }
            }
        }
    })
}

pub fn start_thread_search(
    from_read: Receiver<String>,
    to_write: Sender<String>,
    search_str:&str,
    replace_str:&str,
    only_first:bool
) -> JoinHandle<()> {
    let str_search = String::from(search_str);
    let str_replace = String::from(replace_str);
    let first_only = only_first;
    spawn(move || {
        let mut already_find = false;
        for l in from_read {
            let res:String=
                if l.contains(&str_search)
                {
                    if !already_find || !first_only {
                        already_find = true;
                        replace(&l,&str_search,&str_replace)
                    }else{
                        already_find = true;
                        l
                    }                    
                }else{
                    l
                };
            if to_write.send(res).is_err() {
                println!("error sending to write");
                return;
            }
        }

    })
}

pub fn replace(l:&str,searched:&str,replace:&str)->String{
    if !l.contains(&searched) {return String::from(l);}
    let col_pos = l.find(&searched).unwrap_or_else(|| l.len());
    //cut string in three parts
    let mut line = String::from(l);
    let after = line.split_off(col_pos + searched.len());
    let _s= line.split_off(col_pos);
    let before = line;
    //replace
    let mut replaced= before;
    replaced.push_str(replace);
    replaced.push_str(after.as_str());
    replaced
    //TODO: carefull after can also contain searched 
}

#[test]
fn replace_test() {
    let st1="SearchAndReplace";
    let st2="Search And Replace";
    let st3="SearchedAndReplace";
    assert_eq!(st1, replace(st2,st2,st1));
    assert_eq!(st3, replace(st3,st2,st1));
}

pub fn start_thread_write(
    from_search: Receiver<String>,
    fic_out:&str
) -> JoinHandle<()> {
    println!("TODO : write to {}",fic_out);
    spawn(move || {
        for d in from_search {
            println!("{}",d);
        };
    })
}

pub fn traitement(fic:&str,search_str:&str,replace_str:&str,only_first:bool){
    let mut fic_out = String::from(fic);
    fic_out.push_str(".chg");

    //MPSC chanels
    //read threads to join thread
    let (to_search, from_read) = channel();
    let (to_write, from_search) = channel();

    let hread = start_thread_read(to_search,fic);
    let hsearch = start_thread_search(from_read,to_write,search_str,replace_str,only_first);
    let hwrite = start_thread_write(from_search,&fic_out);

    //wait for threads to stop
    if hread.join().is_err() {
        println!("Thread read finished with error");
    }
    if hsearch.join().is_err() {
        println!("Thread search finished with error");
    }
    if hwrite.join().is_err() {
        println!("Thread write finished with error");
    }
}

fn main() {
    println!("Search and replace");

    let fic = "./src/main.rs";
    let search_str = "Search and replace";
    let replace_str = "Found and replaced";
    let only_first = true;

    traitement(fic,search_str,replace_str,only_first);

    println!("Search and replace All");
    traitement(fic,search_str,replace_str,false);
}