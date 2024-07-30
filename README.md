# Minesweeper Generator

A minesweeper text generator written in Rust. Generates a game of minefield for
Discord's emoji style.

The regular rule set of Minesweeper is implied, but you can add an optional
number of anti-mines or change adjacency rules with flags.

```text
5x5 with 2 mines and 2 anti-mines
- :boom: and :rosette: are mines and anti-mines, meaning you lose
- Zero tiles :blue_square: have no adjacent mines
- Medals :medal: are a numbered combination of mines equaling zero (:first_place: is 1 mine 1 anti, :second_place: is 2 mine 2 anti, etc. to 3, further are generic)
- Number tiles :one: are a positive combination of mines (2 mines and 1 anti-mine is :one:)
- Letter tiles :regional_indicator_a: are a negative combination of mines (1 mine and 2 anti-mines is :regional_indicator_a:)

Adjacency rule set: Adjacent
- Mines will be counted as adjacent if they are 1 tile away in any direction

||:one:||||:one:||||:blue_square:||||:blue_square:||||:blue_square:||
||:boom:||||:one:||||:one:||||:one:||||:one:||
||:first_place:||||:first_place:||||:one:||||:boom:||||:one:||
||:rosette:||||:regional_indicator_b:||||:first_place:||||:first_place:||||:one:||
||:regional_indicator_a:||||:regional_indicator_b:||||:rosette:||||:regional_indicator_a:||||:blue_square:||
```

## Installation

Download the [latest release binary](https://github.com/FireIsGood/minesweeper-generator/releases) or build from the source.

To build from the source, clone the repo and run the cargo command:

```bash
cargo build --release
```

The binary will be created in `./target/release`.

## Usage

By default, a grid of 5x5 with 4 mines is generated. You can use different
options that are found with the `--help` argument.

Settings include:

- Anti-mine string
- Mine string
- Spoiler string
- Width
- Height
- Mine count
- Anti-mine count

Run the binary as normal:

```bash
# Default settings of 5x5 with 4 mines
minesweeper-generator

# Display help
minesweeper-generator --help

# Generate a 10x9 board with 14 mines
minesweeper-generator -W 10 -H 9 -m 14

# Generate a 20x20 board with 200 mines, 200 anti-mines, and knight counting
# --no-limits is used to enable over 200 grid tiles
minesweeper-generator -W 20 -H 20 --no-limits -m 200 -a 200

# Generate a board with a custom mine string
minesweeper-generator --mine-str :fish:
```

## Limitations

Discord only allows for 99 emotes to be rendered in a message, so the program
will not allow you to generate an area of over 90 tiles by default.

If you wish to generate larger boards, use the `--no-limits` flag.

## Contributing

This was written as a meme, but pull requests and issues are welcome!

## License

[MIT](https://choosealicense.com/licenses/mit/)
