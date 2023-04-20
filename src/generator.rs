use anyhow::{Result, bail};
use rand::{rngs::OsRng, RngCore};
use crate::{config::Config, alphabet::Alphabet};

/// Generate password using provided configuration
pub fn generate(cfg: &Config, a: Option<Alphabet>) -> Result<String> {
    let mut password: Vec<char> = Vec::with_capacity(cfg.len);
    let alphabet = if a.is_some() {
        a.unwrap().shuffled()
    } else {
        if cfg.alphabet.len() == 0 {
            bail!("no alphabet or alphabet configuration provided");
        }
        cfg.alphabet.chars().collect()
    };

    for n in 0usize..cfg.len {
        let mut access_index = OsRng.next_u32() as usize % alphabet.len();
        // using alphabet with known size
        let mut new_password_char = alphabet.get(access_index).unwrap();

        // don't allow same symbols together
        if n > 0 && password.get(n-1).unwrap() == new_password_char {
            while new_password_char == password.get(n-1).unwrap() {
                access_index = OsRng.next_u32() as usize % alphabet.len();
                new_password_char = alphabet.get(access_index).unwrap();
            }
        }

        password.push(*new_password_char);
    }

    Ok(String::from_iter(password))
}