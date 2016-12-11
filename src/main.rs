use std::io::prelude::*;
use std::io::{BufReader};
use std::fs::File;
use std::collections::HashMap;

include!("./sentiment.rs");

// Gets a normalized nick from a log nick.
// - Removes beginning '+' and '@'
// - Removes trailing '_' and '-'
fn get_nick(s: &str) -> &str {
    let mut ret = s.trim();

    if ret.starts_with("+") || ret.starts_with("@") {
        ret = &ret[1..ret.len()];
    }

    if ret.ends_with("_") || ret.ends_with("-") {
        ret = &ret[0..ret.len()-1]
    }

    return &ret
}

fn main() {
    let path = "/tmp/log".to_string();
    let f = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", path, why),
        Ok(f) => f,
    };
    let f = BufReader::new(f);

    let mut result = HashMap::new();

    let analizer = sentiment::new("/Users/coornail/rust/ircbot/src/wordlist.txt".to_string());

    let mut i = 0;

    for line in f.lines() {
        // Parse lines.
        let l = line.unwrap();
        let parts = l.split("\t").collect::<Vec<&str>>();
        let who = get_nick(parts[1]).to_string();

        // Skip ignored names.
        if ["*", "**", "***", "--", "---", "-->", "<--"].contains(&who.as_str()) {
            continue;
        }
        i = i+1;

        // Calculate new comment value.
        let comment_value = analizer.analyze(parts[2]);
        let sum_comment_value = match result.get(&who) {
            Some(existing_comment_value) => existing_comment_value + comment_value,
            None => comment_value,
        };
        result.insert(who, sum_comment_value);
    }

    print!("{:?}\n", result);
    print!("{} lines analyzed.\n", i);
}
