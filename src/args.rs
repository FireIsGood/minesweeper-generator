use std::env;

pub struct MinesweeperArguments {
    pub spoiler_char: String,
    pub width: i32,
    pub height: i32,
    pub mine_count: i32,
    pub anti_mine_count: i32,
}

impl MinesweeperArguments {
    pub fn new(
        spoiler_char: String,
        width: i32,
        height: i32,
        mine_count: i32,
        anti_mine_count: i32,
    ) -> Self {
        Self {
            spoiler_char,
            width,
            height,
            mine_count,
            anti_mine_count,
        }
    }
}

pub fn get_args() -> Option<MinesweeperArguments> {
    let args: Vec<String> = env::args().collect();

    let spoiler_char: String = args.get(1)?.into();
    let width: i32 = args.get(2)?.parse().ok()?;
    let height: i32 = args.get(3)?.parse().ok()?;
    let mine_count: i32 = args.get(4)?.parse().ok()?;
    // Default is 0
    let anti_mine_count: i32 = args.get(5).unwrap_or(&"0".to_owned()).parse().ok()?;

    Some(MinesweeperArguments::new(
        spoiler_char,
        width,
        height,
        mine_count,
        anti_mine_count,
    ))
}

pub fn valid_arguments(args: &MinesweeperArguments) -> Result<(), String> {
    if args.anti_mine_count < 0 || args.mine_count < 0 {
        return Err("Negative mines".to_owned());
    }
    if args.width * args.height > 90 {
        return Err("Board too large".to_owned());
    }

    Ok(())
}