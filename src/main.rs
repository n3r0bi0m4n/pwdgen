#![feature(result_option_inspect)]

use alphabet::{Alphabet, AlphabetConfig};
use anyhow::{Result, bail};
use clap::Parser;
use config::Config;
use generator::generate;

mod config;
mod generator;
mod alphabet;

fn main() -> Result<()> {
    let cfg = Config::parse();

    if cfg.len > u8::MAX.into() {
        bail!("maximum password len is {}", u8::MAX);
    }

    if cfg.len == 0 {
        bail!("zero password length");
    }

    let alphabet = Alphabet::new(&AlphabetConfig::new(&cfg));

    generate(&cfg, alphabet).inspect(|pwd| println!("{pwd}"))?;

    Ok(())
}
