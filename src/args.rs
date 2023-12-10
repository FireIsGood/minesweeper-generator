//! Argument parsing logic.
use clap::Parser;

/// Counting rules enum for the different game modes
///
/// The default mode is Adjacent which is the same as a normal game of minesweeper.
#[derive(clap::ValueEnum, Clone, Debug, Default)]
pub enum CountRules {
    #[default]
    Adjacent,
    Knight,
}

/// Argument struct.
///
/// Uses Clap to derive parsing for the arguments. This adds the `--help` sub-command.
#[derive(Parser, Debug)]
#[clap(version)]
pub struct Args {
    /// Adjacency counting rules
    #[arg(short, long, default_value_t, value_enum)]
    pub count_rules: CountRules,

    /// Width of the board
    #[arg(short = 'W', long, default_value_t = 5)]
    pub width: u8,

    /// Height of the board
    #[arg(short = 'H', long, default_value_t = 5)]
    pub height: u8,

    /// Number of mines
    #[arg(short, long, default_value_t = 4)]
    pub mine_count: u8,

    /// Number of anti-mines
    #[arg(short, long, default_value_t = 0)]
    pub anti_mine_count: u8,

    /// String that makes the inside text a spoiler block
    #[arg(short, long, default_value = "||")]
    pub spoiler_str: String,

    /// Uncap board size
    #[arg(long, default_value_t = false)]
    pub no_limits: bool,
}
