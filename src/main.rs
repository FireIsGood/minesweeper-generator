mod args;
mod rules;
mod grid;

use clap::Parser;
use rules::print_rules;

use crate::{
    args::Args,
    grid::{generate_grid, print_grid},
};

fn main() {
    // Generate variables
    let args = Args::parse();
    let grid = generate_grid(&args);

    // Return if the grid is invalid
    if grid.is_none() {
        println!("Grid could not be generated");
        return;
    }

    // Write the output
    print_rules(&args);
    print_grid(grid, args);
}
