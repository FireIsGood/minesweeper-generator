//! Grid generation and printing logic.
use rand::Rng;

use crate::{args::Args, MAX_BOARD_SIZE};

/// Content of a tile as used in the pure grid.
///
/// Tiles are stored as just having mines, anti-mines, or nothing. The grid printing functions
/// handle the mine count numbers.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TileContent {
    Empty,
    Mine,
    AntiMine,
}

/// 2d vector pure grid representation.
///
/// Represents the grid as a 2d vector of `TileContent`.
type MinesweeperGrid = Vec<Vec<TileContent>>;

/// Adjacent mine count.
///
/// Counts mines and anti-mines for translation to string.
type AdjacentMineCount = (i32, i32);

/// Helper function go generate a random grid coordinate.
fn get_random_tile(width: u8, height: u8) -> (usize, usize) {
    let random_width = rand::thread_rng().gen_range(0..width);
    let random_height = rand::thread_rng().gen_range(0..height);
    (random_width as usize, random_height as usize)
}

/// Generates a given event on a random empty tile in the grid.
///
/// This requires that the grid is not full which is handled by the `generate_grid` function.
fn generate_tile(grid: &mut MinesweeperGrid, args: &Args, tile_type: TileContent) {
    loop {
        let (random_width, random_height) = get_random_tile(args.width, args.height);
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

/// Returns an a MinesweeperGrid if the board area requirements are met.
///
/// There must be more grid slots than mines to stop infinite loops when placing mines.
pub fn generate_grid(args: &Args) -> Option<MinesweeperGrid> {
    let board_area = args.width * args.height;
    let mine_total_count = args.mine_count + args.anti_mine_count;

    let too_many_mines: bool = board_area < mine_total_count;
    let over_max_mine_count: bool = !args.no_limits && board_area > MAX_BOARD_SIZE;

    if too_many_mines {
        println!("More mines than grid slots!");
        return None;
    }

    if over_max_mine_count {
        println!(
            "Over {} mines which will not render in Discord, use --no-limits to override.",
            MAX_BOARD_SIZE
        );
        return None;
    }

    let mut grid: MinesweeperGrid =
        vec![vec![TileContent::Empty; args.height as usize]; args.width as usize];

    // Generate the mines
    for _ in 1..=args.mine_count {
        generate_tile(&mut grid, args, TileContent::Mine);
    }
    for _ in 1..=args.anti_mine_count {
        generate_tile(&mut grid, args, TileContent::AntiMine);
    }

    Some(grid)
}

/// Counts adjacent mines at a coordinate given a rule.
///
/// This uses one of the counting rules depending on the arguments given. It is called for each
/// grid coordinate.
fn count_mines(grid: &MinesweeperGrid, x: i32, y: i32, args: &Args) -> AdjacentMineCount {
    match args.count_rules {
        crate::args::CountRules::Adjacent => count_adjacent_mines(grid, x, y),
        crate::args::CountRules::Knight => count_knight_mines(grid, x, y),
    }
}

/// Counts adjacent mines.
///
/// This is the basic 8 adjacent tile rule as in normal minesweeper.
fn count_adjacent_mines(grid: &MinesweeperGrid, x: i32, y: i32) -> AdjacentMineCount {
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

/// Counts knight move adjacent mines.
///
/// This is a special rule that counts tiles that are away by 1 knight move.
fn count_knight_mines(grid: &MinesweeperGrid, x: i32, y: i32) -> AdjacentMineCount {
    let mut mine_count = 0;
    let mut anti_mine_count = 0;

    let coordinate_list = [
        (1, 2), // Top right
        (2, 1),
        (2, -1), // Bottom right
        (1, -2),
        (-1, -2), // Bottom left
        (-2, -1),
        (-2, 1), // Top left
        (-1, 2),
    ];

    for (x_offset, y_offset) in coordinate_list {
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

    (mine_count, anti_mine_count)
}

/// Print a grid if it exists or returns.
///
/// Arguments should match the arguments used to generate the grid.
pub fn print_grid(grid: Option<MinesweeperGrid>, args: Args) {
    if grid.is_none() {
        return;
    }
    let grid = grid.unwrap();

    let mine_str = ":boom:";
    let anti_mine_str = ":rosette:";

    // Temporary basic print
    for y in 0..args.height {
        for x in 0..args.width {
            let tile = grid.get(x as usize).unwrap().get(y as usize).unwrap();
            print!("{}", args.spoiler_str);
            // Tile itself
            match tile {
                TileContent::Mine => print!("{}", mine_str),
                TileContent::AntiMine => print!("{}", anti_mine_str),
                TileContent::Empty => print!(
                    "{}",
                    number_to_emoji(count_mines(&grid, x.into(), y.into(), &args), &args)
                ),
            }
            print!("{}", args.spoiler_str);
        }
        println!();
    }
}

/// Convert a given tile's number to an emoji.
///
/// If anti-mines are in play, unique medals are used for the first four combination zeroes.
fn number_to_emoji((mines, anti_mines): AdjacentMineCount, args: &Args) -> String {
    let total_mines = mines - anti_mines;

    // If playing with anti mines, use extended zeroes
    if args.anti_mine_count != 0 {
        // Case of true zero
        if mines == 0 && anti_mines == 0 {
            return ":blue_square:".to_owned();
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
        0 => ":blue_square:",
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
