use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;
use std::env;

include!("./sentiment.rs");

// Gets a normalized nick from a log nick.
// - Removes beginning '+' and '@'
// - Removes trailing '_' and '-'
fn get_nick(s: &str) -> &str {
    let ignored_prefixes: &[_] = &['+', '@', ' '];
    let ignored_suffixes: &[_] = &['_', '-', ' '];

    return &s.trim_left_matches(ignored_prefixes)
        .trim_right_matches(ignored_suffixes);
}

fn main() {
    // Parse arguments.
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
       panic!("Usage: {} [log file name]", args[0]);
    }
    let path = args[1].clone();

    let f = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", path, why),
        Ok(f) => f,
    };
    let f = BufReader::new(f);

    //let mut result = HashMap::new();

    let analizer = sentiment::new("./src/wordlist.txt".to_string());

    let result = f.lines().map(|line| {
        let line = line.expect("Couldn't parse line");

        let parts = line.split("\t").collect::<Vec<&str>>();
        let who = get_nick(parts[1]).to_string();
        let comment_value = analizer.analyze(parts[2]);

        return (who, comment_value);
    }).fold(HashMap::new(), |mut res, curr| {
        if ["*", "**", "***", "--", "---", "-->", "<--", "-", "", "<-", "=!=", "<"]
            .contains(&curr.0.as_str()) {
                return res;
            }

        let val = match res.get(&curr.0) {
            Some(existing_comment_value) => existing_comment_value + curr.1,
            None => curr.1,
        };

        res.insert(curr.0, val);
        return res;
    });

    for (key, value) in result {
        print!("{} {}\n", key, value);
    }

    //print!("\n{} lines analyzed.\n", i);
}
