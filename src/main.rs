mod args;
mod grid;

use crate::{args::{get_args, valid_arguments}, grid::{generate_grid, print_grid}};


fn main() {
    println!("Minesweeper generator!");
    let expanded_rules = "\
    - :boom: and :rosette: are mines and anti-mines meaning you lose\n\
    - Zero tiles :zero: are actually 0\n\
    - Medals :medal: are a numbered combination of mines equaling zero (:first_place: is 1 mine 1 anti, :second_place: is 2 mine 2 anti, etc. to 3, further are generic)\n\
    - Number tiles :one: are a positive combination of mines (2 mines and 1 anti-mine is :one:)\n\
    - Letter tiles :regional_indicator_a: are a negative combination of mines (1 mine and 2 anti-mines is :regional_indicator_a:)\n\
    ";
    let possible_args = get_args();

    if possible_args.is_none() {
        println!("Not enough arguments!");
        return;
    }
    let args = possible_args.unwrap();

    // Exit for bad arguments
    match valid_arguments(&args) {
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
        Ok(_) => {}
    }

    // Debug info
    // println!("Spoiler character: {}", &args.spoiler_char);
    // println!("Width: {}", &args.width);
    // println!("Height: {}", &args.height);
    // println!("Mines: {}", &args.mine_count);

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

    let grid = generate_grid(&args);

    // println!("{:?}", &grid);
    print_grid(grid, args);

    ()
}
