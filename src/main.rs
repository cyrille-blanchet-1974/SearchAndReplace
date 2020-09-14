use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn replace(l:&str,searched:&str,replace:&str)->String{
    let debug = true;
    let original = String::from(l);
    let col_pos = l.find(&searched).unwrap_or(l.len());
    //cut string in three parts
    let mut line = String::from(l);
    let after = line.split_off(col_pos + searched.len());
    let _s= line.split_off(col_pos);
    let before = line;
    //replace
    let mut replaced= String::from(before);
    //replaced.push_str(before.as_str());
    replaced.push_str(replace);
    replaced.push_str(after.as_str());
    if debug {
        println!("original:{}",original);
        println!("newline :{}",replaced);
    }
    replaced
}

pub fn read(fic: &str, searched_str : &str,replaced_str : &str, first_only:bool) -> bool {
    let mut already_find = false;
    let input = File::open(&fic);
    match input {
        Err(e) => {
            println!("Error reading file {} => {}", &fic, e);
        }
        Ok(f) => {
            let buffered = BufReader::new(f);
            for line in buffered.lines() {
                if let Ok(l) = line {
                    if l.contains(&searched_str)
                    {
                        let res:String;
                        if !already_find{
                            res= replace(&l, searched_str, replaced_str);
                        }else{
                            if !first_only{
                                res= replace(&l, searched_str, replaced_str);
                            }else{
                                res = String::from(l);
                            }
                        } 
                        already_find = true;
                        //TODO : write res line
                    }
                    else
                    {
                        //TODO : write initial line
                    }
                } //TODO : else ...
            }
        }
    }
    false
}

fn main() {
    println!("Search and replace");
    if read("./src/main.rs","Search and replace","Found and replaced",true)
    {
        println!("found")
    }
}
