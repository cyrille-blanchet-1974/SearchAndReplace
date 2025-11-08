use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread::{spawn, JoinHandle};

pub fn start_thread_search(
    from_read: Receiver<String>,
    to_write: Sender<String>,
    search_str: &str,
    replace_str: &str,
    only_first: bool,
) -> JoinHandle<()> {
    let str_search = String::from(search_str);
    let str_replace = String::from(replace_str);
    let first_only = only_first;
    spawn(move || {
        let mut already_find = false;
        for l in from_read {
            let res: String = if l.contains(&str_search) {
                if !already_find || !first_only {
                    already_find = true;
                    replace(&l, &str_search, &str_replace)
                } else {
                    already_find = true;
                    l
                }
            } else {
                l
            };
            if to_write.send(res).is_err() {
                println!("error sending to write");
                return;
            }
        }
    })
}

pub fn replace(l: &str, searched: &str, replaced: &str) -> String {
    if !l.contains(searched) {
        return String::from(l);
    }
    let col_pos = l.find(searched).unwrap_or(l.len());
    //cut string in three parts
    let mut line = String::from(l);
    let mut after = line.split_off(col_pos + searched.len());
    let _s = line.split_off(col_pos);
    let before = line;
    //replace
    let mut res = before;
    res.push_str(replaced);
    //carefull after can also contain searched
    after = replace(&after, searched, replaced);
    res.push_str(after.as_str());
    res
}

#[test]
fn replace_test() {
    let st1 = "SearchAndReplace";
    let st2 = "Search And Replace";
    let st3 = "SearchedAndReplace";
    assert_eq!(st1, replace(st2, st2, st1));
    assert_eq!(st3, replace(st3, st2, st1));
}
