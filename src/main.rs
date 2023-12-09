mod args;
mod grid;

use crate::{
    args::get_args,
    grid::{generate_grid, print_grid},
};

fn main() {
    println!("Minesweeper generator!");
    let expanded_rules = "\
    - :boom: and :rosette: are mines and anti-mines meaning you lose\n\
    - Zero tiles :zero: are actually 0\n\
    - Medals :medal: are a numbered combination of mines equaling zero (:first_place: is 1 mine 1 anti, :second_place: is 2 mine 2 anti, etc. to 3, further are generic)\n\
    - Number tiles :one: are a positive combination of mines (2 mines and 1 anti-mine is :one:)\n\
    - Letter tiles :regional_indicator_a: are a negative combination of mines (1 mine and 2 anti-mines is :regional_indicator_a:)\n\
    ";

    // Generate variables
    let maybe_args = get_args();
    match maybe_args {
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
        Ok(_) => (),
    }
    let args = maybe_args.unwrap();

    let maybe_grid = generate_grid(&args);
    match maybe_grid {
        None => {
            println!("Board too small!");
            return;
        }
        Some(_) => (),
    }
    let grid = maybe_grid.unwrap();

    print!(
        "{}x{} with {} mines",
        args.width, args.height, args.mine_count
    );
    if args.anti_mine_count != 0 {
        println!(" and {} anti-mines", args.anti_mine_count);
        println!("{}", expanded_rules);
    } else {
        println!();
    }

    // println!("{:?}", &grid);
    print_grid(grid, args);

    ()
}
