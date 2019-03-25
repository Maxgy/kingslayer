use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Lexer {
    filter_out: Vec<String>,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            filter_out: vec![
                "a", "an", "at", "go", "my", "of", "that", "the", "through", "to",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        }
    }

    pub fn lex(&self, s: &str) -> Vec<String> {
        let parts = self.filter_parts(s);
        self.mod_words(&parts)
    }

    fn filter_parts(&self, s: &str) -> Vec<String> {
        let mut words: Vec<String> = s.split_whitespace().map(|x| x.to_lowercase()).collect();
        words.retain(|w| !(&self.filter_out).contains(&w));
        words
    }

    fn mod_words(&self, words: &[String]) -> Vec<String> {
        let mut modified = Vec::with_capacity(words.len());
        for w in words {
            modified.push(
                match w.as_str() {
                    "north" => "n",
                    "south" => "s",
                    "east" => "e",
                    "west" => "w",
                    "northeast" => "ne",
                    "northwest" => "nw",
                    "southeast" => "se",
                    "southwest" => "sw",
                    "up" => "u",
                    "down" => "d",
                    "x" => "examine",
                    _ => w,
                }
                .to_string(),
            );
        }
        modified
    }
}
