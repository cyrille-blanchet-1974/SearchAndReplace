use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn read(fic: &str, searched : &str) -> bool {
    let mut already_find = false;
    let input = File::open(&fic);
    match input {
        Err(e) => {
            println!("Error reading file {} => {}", &fic, e);
        }
        Ok(f) => {
            let buffered = BufReader::new(f);
            let mut line_pos = 0;
            for line in buffered.lines() {
                if let Ok(l) = line {
                    line_pos+=1;
                    //TODO if already found and change only one 
                    //then just copy others datas
                    if l.contains(&searched)
                    {
                        println!("Found at line {}",line_pos);
                        if already_find{
                            println!("not the first time");
                        }
                        already_find = true;
                        let col_pos = l.find(&searched).unwrap_or(l.len());
                        println!("Found at column {}",col_pos +1);
                        println!("{}",l);
                        println!("123456789012345678901234567890123456789012345678901234567890");
                        //TODO : replace
                        //TODO : write changed line
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
    if read("./src/main.rs","Search and replace")
    {
        println!("found")
    }
}
