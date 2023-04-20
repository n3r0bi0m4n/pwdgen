use clap::Parser;

const DEFAULT_PASSWORD_LEN: usize = 48;

/// Main configuration
#[derive(Debug, Parser)]
#[command(next_line_help = true, arg_required_else_help = true)]
#[command(about = "Utility to generate passwords", version)]
pub struct Config {
    /// Password length
    #[arg(long, default_value_t = DEFAULT_PASSWORD_LEN)]
    pub len: usize,

    /// a-z
    #[arg(long, default_value_t = false)]
    pub use_small_alpha: bool,

    /// A-Z
    #[arg(long, default_value_t = false)]
    pub use_big_alpha: bool,

    /// 0-9
    #[arg(long, default_value_t = false)]
    pub use_numeric: bool,

    /// !@#$%^&*
    #[arg(long, default_value_t = false)]
    pub use_num_symbols: bool,

    /// -_
    #[arg(long, default_value_t = false)]
    pub use_dashes: bool,

    /// ()[]{}<>
    #[arg(long, default_value_t = false)]
    pub use_braces: bool,
    
    /// +=;\\|/?.,
    #[arg(long, default_value_t = false)]
    pub use_other_symbols: bool,

    /// All symbols
    #[arg(long, default_value_t = false)]
    pub use_all: bool,

    /// User-defined alphabet
    #[arg(long, default_value_t = String::new())]
    pub alphabet: String,
}