pub mod sentiment {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::{self, BufReader};
    use std::path::Path;


    pub struct Analizer {
        wordlist: HashMap<String, i8>
    }

    impl Analizer {
        pub fn analyze(self, s: &str) -> f32 {
            return 0.0;
        }
    }

    pub fn new(path: String) -> Analizer {
        let mut wordlist = HashMap::new();

        let mut f = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", path, why),
            Ok(f) => f,
        };

        let f = BufReader::new(f);

        for line in f.lines() {
            let l = line.unwrap();
            let parts = l.split("\t").collect::<Vec<&str>>();

            let word = parts[0].to_string();
            let score = parts[1].parse::<i8>().unwrap();

            wordlist.insert(word, score);
        }

        println!("{:?}", wordlist);
        return Analizer {
            wordlist: wordlist
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sentiment_test() {
        /*
           analyze("I am happy"); //Score: 3, Comparative: 1
           analyze("I am so happy"); //Score: 6, Comparative: 1.5
           analyze("I am extremely happy"); //Score: 12, Comparative: 3
           analyze("I am really sad"); //Score: -4, Comparative: -1
*/
        let mut analizer = sentiment::new("/Users/coornail/rust/ircbot/src/wordlist.txt".to_string());
        assert!(analizer.analyze("Hey you worthless scumbag") == -1.5)
    }
}
