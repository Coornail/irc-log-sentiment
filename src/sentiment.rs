use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub struct Analizer {
    wordlist: HashMap<String, i8>,
}

impl Analizer {
    pub fn analyze(&self, s: &str) -> f32 {
        let tokens = s.split(" ");
        let word_count = tokens.clone().count();

        let score = tokens.map(|word| word.to_string().to_lowercase())
            .map(|word| match self.wordlist.get(&word) {
                Some(word_score) => *word_score,
                None => 0 as i8,
            })
            .fold(0, |sum, curr| sum + curr);

        return score as f32 / word_count as f32;
    }
}

pub fn new(path: &str) -> Analizer {
    let mut wordlist = HashMap::new();

    let f = match File::open(&path) {
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

    return Analizer { wordlist: wordlist };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sentiment_test() {
        let sent = new("./src/wordlist.txt");
        assert!(sent.analyze("Hey you worthless scumbag") == -1.5);
        assert!(sent.analyze("I am happy") == 1 as f32);
        assert!(sent.analyze("I am so happy") == 0.75);
        assert!(sent.analyze("I am extremely happy") == 0.75 as f32);
        assert!(sent.analyze("I am really sad") == -0.5 as f32);
    }
}
