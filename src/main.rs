mod args;
mod grid;
mod rules;

use crate::{
    args::Args,
    grid::{generate_grid, print_grid},
    rules::print_rules,
};

use clap::Parser;

/// Maximum board size that will be allowed by default.
///
/// This can be overridden by the user with the flag `--no-limits` if a different messaging app is
/// used.
///
/// Discord max emoji size is 99, so 90 is a soft limit on grid size to stop users from generating
/// boards that cannot be sent.
const MAX_BOARD_SIZE: u8 = 90;

/// Main entry from the command line. This does all the parsing and printing.
///
/// Parses the CLI arguments and prints a game if it satisfies the rules.
fn main() {
    // Generate variables
    let args = Args::parse();
    let grid = generate_grid(&args);

    // Return if the grid is invalid
    if grid.is_none() {
        return;
    }

    // Write the output
    print_rules(&args);
    print_grid(grid, args);
}
