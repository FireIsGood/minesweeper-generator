use std::{env, fmt};

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

#[derive(Debug)]
pub enum ArgError {
    NoSpoilerChar,
    NoWidth,
    NoHeight,
    NoMines,
    NonNumericSize,
    NonNumericMineCount,
    NegativeMineCount,
    BoardTooSmall,
}

impl fmt::Display for ArgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error_message = match self {
            ArgError::NoSpoilerChar => "no spoiler character provided",
            ArgError::NoWidth => "no width provided",
            ArgError::NoHeight => "no height provided",
            ArgError::NoMines => "no mine count provided",
            ArgError::NonNumericSize => "size is not a number",
            ArgError::NonNumericMineCount => "mine count is not a number",
            ArgError::NegativeMineCount => "mine count is negative",
            ArgError::BoardTooSmall => "board is not large enough for mines",
        };
        write!(f, "{}", error_message)
    }
}

pub fn get_args() -> Result<MinesweeperArguments, ArgError> {
    let args: Vec<String> = env::args().collect();

    let spoiler_char: String = args.get(1).ok_or(ArgError::NoSpoilerChar)?.into();
    let width: i32 = args
        .get(2)
        .ok_or(ArgError::NoWidth)?
        .parse()
        .map_err(|_| ArgError::NonNumericSize)?;
    let height: i32 = args
        .get(3)
        .ok_or(ArgError::NoHeight)?
        .parse()
        .map_err(|_| ArgError::NonNumericSize)?;
    let mine_count: i32 = args
        .get(4)
        .ok_or(ArgError::NoMines)?
        .parse()
        .map_err(|_| ArgError::NonNumericMineCount)?;
    // Default is 0
    let anti_mine_count: i32 = args
        .get(5)
        .unwrap_or(&"0".to_owned())
        .parse()
        .map_err(|_| ArgError::NonNumericMineCount)?;

    if anti_mine_count < 0 || mine_count < 0 {
        return Err(ArgError::NegativeMineCount);
    }
    if width * height > 90 {
        return Err(ArgError::BoardTooSmall);
    }

    Ok(MinesweeperArguments::new(
        spoiler_char,
        width,
        height,
        mine_count,
        anti_mine_count,
    ))
}
