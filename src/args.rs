use clap::Parser;

#[derive(clap::ValueEnum, Clone, Debug, Default)]
pub enum CountRules {
    #[default]
    Adjacent,
    Knight,
}

#[derive(Parser, Debug)]
#[clap(version)]
pub struct Args {
    /// String that makes the inside text clickable
    #[arg(short, long, default_value = "||")]
    pub spoiler_str: String,

    #[arg(short, long, default_value_t, value_enum)]
    pub count_rules: CountRules,

    /// Width of the board
    #[arg(short = 'W', long)]
    pub width: u8,

    /// Height of the board
    #[arg(short = 'H', long)]
    pub height: u8,

    /// Number of mines
    #[arg(short, long)]
    pub mine_count: u8,

    /// Number of anti-mines
    #[arg(short, long, default_value_t = 0)]
    pub anti_mine_count: u8,
}
