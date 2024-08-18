mod args;
mod grid;
mod rules;

use crate::{
    args::Args,
    grid::{generate_grid, print_grid},
    rules::print_rules,
};

use clap::Parser;

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
