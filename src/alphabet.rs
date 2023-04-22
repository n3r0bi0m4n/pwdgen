use rand::seq::SliceRandom;
use crate::config::Config;

// Keep order
pub static SMALL_ALPHA: &str = "abcdefghijklmnopqrstuvwxyz";
pub static BIG_ALPHA: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub static NUMERIC: &str = "0123456789";
pub static DASHES: &str = "-_";
pub static NUM_SYMBOLS: &str = "!@#$%^&*";
pub static BRACES: &str = "()[]{}<>";
pub static OTHER_SYMBOLS: &str = "+=;\\|/?.,";

/// Alphabet used to generate passwords
#[derive(Debug)]
pub struct Alphabet(Vec<char>);

impl Alphabet {
    pub fn new(config: &Config) -> Self {
        if !config.alphabet.is_empty() {
            return Self::from(&config.alphabet);
        }

        let mut inner = String::new();

        if config.use_all || config.use_small_alpha {
            inner.push_str(SMALL_ALPHA);
        }

        if config.use_all || config.use_big_alpha {
            inner.push_str(BIG_ALPHA);
        }

        if config.use_all || config.use_numeric {
            inner.push_str(NUMERIC);
        }

        if config.use_all || config.use_dashes {
            inner.push_str(DASHES);
        }

        if config.use_all || config.use_num_symbols {
            inner.push_str(NUM_SYMBOLS);
        }

        if config.use_all || config.use_braces {
            inner.push_str(BRACES);
        }

        if config.use_all || config.use_other_symbols {
            inner.push_str(OTHER_SYMBOLS);
        }

        Self::from(&inner)
    }

    pub fn shuffle(mut self) -> Self {
        self.0.shuffle(&mut rand::rngs::OsRng);
        Self(self.0)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Option<&char> {
        self.0.get(index)
    }
}

impl FromIterator<char> for Alphabet {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        Self(iter.into_iter().collect::<Vec<char>>())
    }
}

impl From<&String> for Alphabet {
    fn from(value: &String) -> Self {
        Self(value.chars().collect())
    }
}
