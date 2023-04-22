#![feature(result_option_inspect)]

use alphabet::Alphabet;
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
        bail!("maximum password length is {}", u8::MAX);
    }

    if cfg.len == 0 {
        bail!("zero password length");
    }

    generate(
        &cfg, 
        Alphabet::new(&cfg)
    ).inspect(
        |pwd| println!("{pwd}")
    )?;

    Ok(())
}
