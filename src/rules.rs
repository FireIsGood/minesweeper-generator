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
        - {} are regular mines and {} are anti-mines, meaning you lose if you didn't deduce it (feel free to click if you know it's a mine!)\n\
        - Blank tiles :blue_square: have no adjacent mines\n\
        - Medals :medal:, :second_place:, etc. are a numbered combination of the same number of regular and anti-mines (:first_place: is 1 regular 1 anti, :second_place: is 2 regular 2 anti, etc. to 3, further are generic)\n\
        - Number tiles :one:, :two:, etc. are a positive combination of mines (2 regular and 1 mine is :one:, 1 regular is :one:, 2 regular is :two:)\n\
        - Letter tiles :regional_indicator_a:, :regional_indicator_b:, etc. are a negative combination of mines (1 regular and 2 anti is :regional_indicator_a:, 1 anti is :regional_indicator_a:, 2 anti is :regional_indicator_b:)\
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
