# Minesweeper Generator

A minesweeper text generator written in Rust. Generates a game of minefield for
Discord's emoji style.

The regular rule set of Minesweeper is implied, but you can add an optional
number of anti-mines.

Unformatted output for a 6x6 grid with 3 mines and 4 anti-mines:

```text
Minesweeper generator!
6x6 with 3 mines and 4 anti-mines
- Zero tiles :zero: are actually 0
- Medals :medal: are a numbered combination of mines equaling zero (:first_place: is 1 mine 1 anti, :second_place: is 2 mine 2 anti, etc. to 3, further are generic)
- Number tiles :one: are a positive combination of mines
- Letter tiles :regional_indicator_a: are a negative combination of mines

||:rosette:||||:first_place:||||:boom:||||:one:||||:zero:||||:zero:||
||:regional_indicator_a:||||:first_place:||||:first_place:||||:first_place:||||:regional_indicator_a:||||:zero:||
||:zero:||||:one:||||:first_place:||||:rosette:||||:regional_indicator_a:||||:first_place:||
||:zero:||||:one:||||:boom:||||:second_place:||||:boom:||||:rosette:||
||:zero:||||:one:||||:first_place:||||:rosette:||||:regional_indicator_a:||||:first_place:||
||:zero:||||:zero:||||:regional_indicator_a:||||:regional_indicator_a:||||:regional_indicator_a:||||:zero:||
```

## Installation

Download the [latest release binary](https://github.com/FireIsGood/minesweeper-generator/releases) or build from the source.

To build from the source, clone the repo and run the cargo command:

```bash
cargo build --release
```

The binary will be created in `./target/release`.

## Usage

Run the binary with the following arguments:

```bash
minesweeper-generator [spoiler text] [width] [height] [mine count] [anti mine count?]
```

## Contributing

This was written as a meme, but pull requests and issues are welcome!

## License

[MIT](https://choosealicense.com/licenses/mit/)
