#![allow(unused)]
use std::env;

use rand::{seq::SliceRandom, Rng};

#[derive(Clone, Copy, Debug, PartialEq)]
enum TileContent {
    Empty,
    Mine,
}

struct MinesweeperArguments {
    spoiler_char: String,
    width: u8,
    height: u8,
    mine_count: u8,
}

type MinesweeperGrid = Vec<Vec<TileContent>>;

fn get_args() -> Option<MinesweeperArguments> {
    let args: Vec<String> = env::args().collect();

    let spoiler_char: String = args.get(1)?.into();
    let width: u8 = args.get(2)?.parse().ok()?;
    let height: u8 = args.get(3)?.parse().ok()?;
    let mine_count: u8 = args.get(4)?.parse().ok()?;

    Some(MinesweeperArguments {
        spoiler_char,
        width,
        height,
        mine_count,
    })
}

fn generate_grid(args: &MinesweeperArguments) -> MinesweeperGrid {
    let mut grid: MinesweeperGrid =
        vec![vec![TileContent::Empty; args.height.into()]; args.width.into()];

    // Generate the mines
    for _ in 1..=args.mine_count {
        loop {
            let random_width: usize = rand::thread_rng().gen_range(0..args.width).into();
            let random_height: usize = rand::thread_rng().gen_range(0..args.height).into();
            let random_tile = grid[random_width][random_height];
            match random_tile {
                TileContent::Mine => continue,
                TileContent::Empty => {
                    grid[random_width][random_height] = TileContent::Mine;
                    break;
                }
            }
        }
    }

    grid
}
fn count_adjacent_mines(grid: &MinesweeperGrid, x: i32, y: i32) -> i32 {
    let mut count = 0;
    for x_offset in -1..=1 {
        for y_offset in -1..=1 {
            let x_check = x - x_offset;
            let y_check = y - y_offset;
            match grid
                .get(x_check as usize)
                .and_then(|g: &Vec<TileContent>| g.get(y_check as usize))
            {
                Some(TileContent::Mine) => count += 1,
                _ => {}
            }
        }
    }
    count
}

fn print_grid(grid: MinesweeperGrid, args: MinesweeperArguments) {
    let mine_str = ":boom:";
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
                TileContent::Empty => print!(
                    "{}",
                    number_to_emoji(count_adjacent_mines(&grid, x.into(), y.into()))
                ),
            }
            print!("{}", args.spoiler_char);
        }
        print!("\n");
    }
}

fn number_to_emoji(number: i32) -> String {
    let converted_str = match number {
        0 => ":zero:",
        1 => ":one:",
        2 => ":two:",
        3 => ":three:",
        4 => ":four:",
        5 => ":five:",
        6 => ":six:",
        7 => ":seven:",
        8 => ":eight:",
        _ => "?",
    };

    converted_str.to_owned()
}

fn main() {
    println!("Minesweeper generator!");
    let possible_args = get_args();

    if possible_args.is_none() {
        println!("Not enough arguments!");
        return;
    }
    let args = possible_args.unwrap();

    // println!("Spoiler character: {}", &args.spoiler_char);
    // println!("Width: {}", &args.width);
    // println!("Height: {}", &args.height);
    // println!("Mines: {}", &args.mine_count);
    println!(
        "{}x{} with {} mines",
        args.width, args.height, args.mine_count
    );

    let grid = generate_grid(&args);

    // println!("{:?}", &grid);
    print_grid(grid, args);

    ()
}
