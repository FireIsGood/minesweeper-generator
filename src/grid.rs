use rand::Rng;

use crate::args::Args;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TileContent {
    Empty,
    Mine,
    AntiMine,
}

type MinesweeperGrid = Vec<Vec<TileContent>>;

pub fn get_random_tile(width: u8, height: u8) -> (usize, usize) {
    let random_width = rand::thread_rng().gen_range(0..width);
    let random_height = rand::thread_rng().gen_range(0..height);
    (random_width as usize, random_height as usize)
}

pub fn generate_tile(grid: &mut MinesweeperGrid, args: &Args, tile_type: TileContent) {
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

pub fn generate_grid(args: &Args) -> Option<MinesweeperGrid> {
    // Make sure it's possible to place all mines
    let board_area = args.width * args.height;
    let mine_total_count = args.mine_count + args.anti_mine_count;
    if board_area <= mine_total_count {
        return None;
    }

    let mut grid: MinesweeperGrid =
        vec![vec![TileContent::Empty; args.height as usize]; args.width as usize];

    // Generate the mines
    for _ in 1..=args.mine_count {
        generate_tile(&mut grid, &args, TileContent::Mine);
    }
    for _ in 1..=args.anti_mine_count {
        generate_tile(&mut grid, &args, TileContent::AntiMine);
    }

    Some(grid)
}

/// Counts adjacent mines at a coordinate given a rule
fn count_mines(grid: &MinesweeperGrid, x: i32, y: i32, args: &Args) -> (i32, i32) {
    match args.count_rules {
        crate::args::CountRules::Adjacent => count_adjacent_mines(grid, x, y),
        crate::args::CountRules::Knight => count_knight_mines(grid, x, y),
    }
}

/// Counts adjacent mines
fn count_adjacent_mines(grid: &Vec<Vec<TileContent>>, x: i32, y: i32) -> (i32, i32) {
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

/// Counts knight step mines
fn count_knight_mines(grid: &MinesweeperGrid, x: i32, y: i32) -> (i32, i32) {
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

/// Print a grid if it exists
pub fn print_grid(grid: Option<MinesweeperGrid>, args: Args) {
    if grid.is_none() {
        return;
    }
    let grid = grid.unwrap();

    let mine_str = ":boom:";
    let anti_mine_str = ":rosette:";

    // Temporary basic print
    for x in 0..args.width {
        for y in 0..args.height {
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
        print!("\n");
    }
}

fn number_to_emoji((mines, anti_mines): (i32, i32), args: &Args) -> String {
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
