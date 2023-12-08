use std::env;

use rand::{seq::SliceRandom, Rng};

#[derive(Clone, Copy, Debug, PartialEq)]
enum TileContent {
    Empty,
    Mine,
    AntiMine,
}

struct MinesweeperArguments {
    spoiler_char: String,
    width: u8,
    height: u8,
    mine_count: u8,
    anti_mine_count: u8,
}

type MinesweeperGrid = Vec<Vec<TileContent>>;

fn get_args() -> Option<MinesweeperArguments> {
    let args: Vec<String> = env::args().collect();

    let spoiler_char: String = args.get(1)?.into();
    let width: u8 = args.get(2)?.parse().ok()?;
    let height: u8 = args.get(3)?.parse().ok()?;
    let mine_count: u8 = args.get(4)?.parse().ok()?;
    // Default is 0
    let anti_mine_count: u8 = args.get(5).unwrap_or(&"0".to_owned()).parse().ok()?;

    Some(MinesweeperArguments {
        spoiler_char,
        width,
        height,
        mine_count,
        anti_mine_count,
    })
}

fn get_random_tile(grid: &MinesweeperGrid, width: u8, height: u8) -> (usize, usize) {
    let random_width: usize = rand::thread_rng().gen_range(0..width).into();
    let random_height: usize = rand::thread_rng().gen_range(0..height).into();
    (random_width, random_height)
}

fn generate_tile(grid: &mut MinesweeperGrid, args: &MinesweeperArguments, tile_type: TileContent) {
    loop {
        let (random_width, random_height) = get_random_tile(&grid, args.width, args.height);
        let random_tile = grid[random_width][random_height];
        match random_tile {
            TileContent::Empty => {
                grid[random_width][random_height] = tile_type;
                break;
            }
            _ => continue,
        }
    }
}

fn generate_grid(args: &MinesweeperArguments) -> MinesweeperGrid {
    let mut grid: MinesweeperGrid =
        vec![vec![TileContent::Empty; args.height.into()]; args.width.into()];

    // Generate the mines
    for _ in 1..=args.mine_count {
        generate_tile(&mut grid, &args, TileContent::Mine);
    }
    for _ in 1..=args.anti_mine_count {
        generate_tile(&mut grid, &args, TileContent::AntiMine);
    }

    grid
}

fn count_adjacent_mines(grid: &MinesweeperGrid, x: i32, y: i32) -> (i32, i32) {
    let mut mine_count = 0;
    let mut anti_mine_count = 0;
    for x_offset in -1..=1 {
        for y_offset in -1..=1 {
            let x_check = x - x_offset;
            let y_check = y - y_offset;
            match grid
                .get(x_check as usize)
                .and_then(|g: &Vec<TileContent>| g.get(y_check as usize))
            {
                Some(TileContent::Mine) => mine_count += 1,
                Some(TileContent::AntiMine) => anti_mine_count += 1,
                _ => {}
            }
        }
    }
    (mine_count, anti_mine_count)
}

fn print_grid(grid: MinesweeperGrid, args: MinesweeperArguments) {
    let mine_str = ":boom:";
    let anti_mine_str = ":rosette:";
    // Temporary basic print
    for x in 0..args.width {
        for y in 0..args.height {
            let tile = grid
                .get::<usize>(x.into())
                .unwrap()
                .get::<usize>(y.into())
                .unwrap();
            print!("{}", args.spoiler_char);
            // Tile itself
            match tile {
                TileContent::Mine => print!("{}", mine_str),
                TileContent::AntiMine => print!("{}", anti_mine_str),
                TileContent::Empty => print!(
                    "{}",
                    number_to_emoji(count_adjacent_mines(&grid, x.into(), y.into()), &args)
                ),
            }
            print!("{}", args.spoiler_char);
        }
        print!("\n");
    }
}

fn number_to_emoji((mines, anti_mines): (i32, i32), args: &MinesweeperArguments) -> String {
    let total_mines = mines - anti_mines;

    // If playing with anti mines, use extended zeroes
    if args.anti_mine_count != 0 {
        // Case of true zero
        if mines == 0 && anti_mines == 0 {
            return ":zero:".to_owned();
        }

        // Case of false zero
        if total_mines == 0 {
            let alternate_zero = match mines {
                1 => ":first_place:",
                2 => ":second_place:",
                3 => ":third_place:",
                _ => ":medal:",
            };
            return alternate_zero.to_owned();
        }
    }

    // Case of number
    let converted_str = match total_mines {
        ..=-9 | 9.. => "?",
        -8 => ":regional_indicator_h:",
        -7 => ":regional_indicator_g:",
        -6 => ":regional_indicator_f:",
        -5 => ":regional_indicator_e:",
        -4 => ":regional_indicator_d:",
        -3 => ":regional_indicator_c:",
        -2 => ":regional_indicator_b:",
        -1 => ":regional_indicator_a:",
        0 => ":zero:",
        1 => ":one:",
        2 => ":two:",
        3 => ":three:",
        4 => ":four:",
        5 => ":five:",
        6 => ":six:",
        7 => ":seven:",
        8 => ":eight:",
    };

    converted_str.to_owned()
}

fn main() {
    println!("Minesweeper generator!");
    let expanded_rules = "\
    - Zero tiles :zero: are actually 0\n\
    - Medals :medal: are a numbered combination of mines equaling zero (:first_place: is 1 mine 1 anti, :second_place: is 2 mine 2 anti, etc. to 3, further are generic)\n\
    - Number tiles :one: are a positive combination of mines\n\
    - Letter tiles :regional_indicator_a: are a negative combination of mines\n\
    ";
    let possible_args = get_args();

    if possible_args.is_none() {
        println!("Not enough arguments!");
        return;
    }
    let args = possible_args.unwrap();

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
