# Quake Log Parser

Simple cli utility to parse quake log files.

## Installation
```
git clone https://github.com/dcfreire/quake-log-parser.git
cd quake-log-parser/
cargo install --path .
```
Cargo will build the quake-log-parser binary and place it in `$HOME/.local/share/cargo/bin/quake-log-parser`.

## Usage

```
$ quake-log-parser --help
Parse quake log files

Usage: quake-log-parser [OPTIONS] <LOG_FILEPATH>

Arguments:
  <LOG_FILEPATH>  Path to the quake log file

Options:
  -d, --death-report       Include a death report on the causes of death of each match
  -g, --game-id <GAME_ID>  Select only specific game from file
  -h, --help               Print help

$ quake-log-parser tests/test.log -g 5 -d
game_5: {
  "total_kills": 14,
  "players": [
    "Zeh",
    "Isgalamido",
    "Assasinu Credi"
  ],
  "kills": {
    "Zeh": 1,
    "Isgalamido": 2,
    "Assasinu Credi": -3
  },
  "kills_by_means": {
    "MOD_ROCKET_SPLASH": 4,
    "MOD_TRIGGER_HURT": 5,
    "MOD_ROCKET": 4,
    "MOD_RAILGUN": 1
  }
}
```
