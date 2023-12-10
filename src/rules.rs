use crate::args::Args;

/// Print the rules for the given argument's rule set
pub fn print_rules(args: &Args) {
    print_mine_count(&args);
    print_mine_rules(&args);
    println!();
    print_adjacency_rules(&args);
    println!();
}

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

fn print_mine_rules(args: &Args) {
    let basic_rules = "\
    - :boom: is a mine, meaning you lose\n\
    - Zero tiles :blue_square: have no adjacent mines\
    ";
    let expanded_rules = "\
    - :boom: and :rosette: are mines and anti-mines, meaning you lose\n\
    - Zero tiles :blue_square: have no adjacent mines\n\
    - Medals :medal: are a numbered combination of mines equaling zero (:first_place: is 1 mine 1 anti, :second_place: is 2 mine 2 anti, etc. to 3, further are generic)\n\
    - Number tiles :one: are a positive combination of mines (2 mines and 1 anti-mine is :one:)\n\
    - Letter tiles :regional_indicator_a: are a negative combination of mines (1 mine and 2 anti-mines is :regional_indicator_a:)\
    ";

    let has_anti_mines: bool = args.anti_mine_count > 0;
    let rules = match has_anti_mines {
        true => basic_rules,
        false => expanded_rules,
    };

    println!("{}", rules);
}

fn print_adjacency_rules(args: &Args) {
    let explanation = match args.count_rules {
        crate::args::CountRules::Adjacent => "1 tile away in any direction",
        crate::args::CountRules::Knight => "1 knight move away",
    };

    println!("Adjacency rule set: {:?}", args.count_rules);
    println!(
        "- Mines will be counted as adjacent if they are {}",
        explanation
    );
}
