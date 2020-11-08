mod paramcli;
mod read;
mod replace;
mod write;

use paramcli::*;
use read::*;
use replace::*;
use std::fs::*;
use std::sync::mpsc::channel;
use write::*;

pub fn traitement(p: &Paramcli) {
    let mut fic_out = String::from(&p.file);
    fic_out.push_str(".chg");

    //MPSC chanels
    let (to_search, from_read) = channel();
    let (to_write, from_search) = channel();

    let hread = start_thread_read(to_search, &p.file);
    let hsearch = start_thread_search(from_read, to_write, &p.search, &p.replace, p.only_first);
    let hwrite = start_thread_write(from_search, &fic_out);

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

    if p.keep_old {
        let mut fic_old = String::from(&p.file);
        fic_old.push_str(".old");
        if rename(&p.file, &fic_old).is_err() {
            println!("error renaming {} to {} aborting", &p.file, fic_old);
            return;
        }
    } else if remove_file(&p.file).is_err() {
        println!("error removing {} aborting", &p.file);
        return;
    }
    if rename(&fic_out, &p.file).is_err() {
        println!("error renaming {} to {} aborting", fic_out, &p.file);
        return;
    }
}

fn main() {
    let param = Paramcli::new();
    traitement(&param);
}
