//! Rules printing logic.
use crate::args::Args;

/// Print the rules for the given argument's rule set.
///
/// Mine count will only display anti-mines if they exist, other rules will be hidden unless they
/// are relevant.
///
/// The adjacency rules prints the rules of the current mode.
pub fn print_rules(args: &Args) {
    print_mine_count(args);
    print_mine_rules(args);
    println!();
    print_adjacency_rules(args);
    println!();
}

/// Mine count printing.
///
/// Writes the board size with the number of mines. Anti-mines are not mentioned unless they are
/// included.
fn print_mine_count(args: &Args) {
    print!(
        "{}x{} with {} mines",
        args.width, args.height, args.mine_count
    );
    if args.anti_mine_count != 0 {
        println!(" and {} anti-mines", args.anti_mine_count);
    } else {
        println!();
    }
}

/// Mine rules printing.
///
/// Prints the rules of the game depending on if anti-mines are included.
fn print_mine_rules(args: &Args) {
    let basic_rules = format!(
        "\
    - {} is a mine, meaning you lose\n\
    - Zero tiles :blue_square: have no adjacent mines\
    ",
        args.mine_str,
    );
    let expanded_rules = format!("\
        - {} and {} are mines and anti-mines, meaning you lose\n\
        - Blank tiles :blue_square: have no adjacent mines\n\
        - Medals :medal: are a numbered combination of mines equaling zero (:first_place: is 1 mine 1 anti, :second_place: is 2 mine 2 anti, etc. to 3, further are generic)\n\
        - Number tiles :one: are a positive combination of mines (2 mines and 1 anti-mine is :one:)\n\
        - Letter tiles :regional_indicator_a: are a negative combination of mines (1 mine and 2 anti-mines is :regional_indicator_a:)\
        ",
            args.mine_str,
            args.anti_mine_str,
    );

    let has_anti_mines: bool = args.anti_mine_count > 0;
    let rules = match has_anti_mines {
        true => &expanded_rules,
        false => &basic_rules,
    };

    println!("{}", rules);
}

/// Adjacency rules printing.
///
/// Rules are found by searching for the `count_rules` in the arguments.
fn print_adjacency_rules(args: &Args) {
    let explanation = match args.count_rules {
        crate::args::CountRules::Adjacent => "1 tile away in any direction",
        crate::args::CountRules::Knight => "1 knight move away",
    };

    println!("Adjacency rule set: {:?}", args.count_rules);
    println!(
        "- Mines will be counted as adjacent **ONLY** if they are {}",
        explanation
    );
}
