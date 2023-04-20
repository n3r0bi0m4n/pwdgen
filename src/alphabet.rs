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

/// Alphabet configuration bitfield
pub struct AlphabetConfig(u8);

impl AlphabetConfig {
    pub fn new(cfg: &Config) -> Self {
        let mut v = 0u8;
        
        if cfg.alphabet.len() > 0 {
            return Self(0);
        }

        if cfg.use_all {
            return Self(127);
        }
        
        if cfg.use_small_alpha {
            v += 1 << 0;
        }

        if cfg.use_big_alpha {
            v += 1 << 1;
        }

        if cfg.use_numeric {
            v += 1 << 2;
        }

        if cfg.use_dashes {
            v += 1 << 3;
        }

        if cfg.use_num_symbols {
            v += 1 << 4;
        }

        if cfg.use_braces {
            v += 1 << 5;
        }

        if cfg.use_other_symbols {
            v += 1 << 6;
        }

        Self(v)
    }

    fn chk(&self, index: u8) -> bool {
        assert!(index < 8);
        (&self.0 & (1 << index)) != 0
    }

    pub fn is_small_alpha(&self) -> bool {
        self.chk(0)
    }

    pub fn is_big_alpha(&self) -> bool {
        self.chk(1)
    }

    pub fn is_numeric(&self) -> bool {
        self.chk(2)
    }

    pub fn is_dashes(&self) -> bool {
        self.chk(3)
    }

    pub fn is_num_symbols(&self) -> bool {
        self.chk(4)
    }

    pub fn is_braces(&self) -> bool {
        self.chk(5)
    }

    pub fn is_other_symbols(&self) -> bool {
        self.chk(6)
    }
}

/// Alphabet used to generate passwords
#[derive(Debug)]
pub struct Alphabet(String);

impl Alphabet {
    pub fn new(config: &AlphabetConfig) -> Option<Self> {
        if config.0 == 0 {
            return None
        }

        let mut inner = String::new();

        if config.is_small_alpha() {
            inner.push_str(SMALL_ALPHA);
        }

        if config.is_big_alpha() {
            inner.push_str(BIG_ALPHA);
        }

        if config.is_numeric() {
            inner.push_str(NUMERIC);
        }

        if config.is_dashes() {
            inner.push_str(DASHES);
        }

        if config.is_num_symbols() {
            inner.push_str(NUM_SYMBOLS);
        }

        if config.is_braces() {
            inner.push_str(BRACES);
        }

        if config.is_other_symbols() {
            inner.push_str(OTHER_SYMBOLS);
        }

        Some(Self(inner))
    }

    pub fn shuffled(&self) -> Vec<char> {
        let mut chars: Vec<char> = self.0.chars().into_iter().collect();
        chars.shuffle(&mut rand::rngs::OsRng);
        chars
    }
}