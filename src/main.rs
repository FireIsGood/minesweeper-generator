mod args;
mod grid;
mod rules;

use clap::Parser;

use crate::{
    args::Args,
    grid::{generate_grid, print_grid},
    rules::print_rules,
};

const MAX_BOARD_SIZE: u8 = 90;

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
