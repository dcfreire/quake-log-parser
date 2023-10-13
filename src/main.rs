use std::fs::File;

use clap::Parser;
use quake_log_parser::parse_games;

/// Parse quake log files
#[derive(Parser)]
struct Cli {
    /// Path to the quake log file
    log_filepath: String,
    /// Include a death report on the causes of death of each match
    #[clap(long, short, action)]
    death_report: bool,
    /// Select only specific game from file
    #[clap(long, short)]
    game_id: Option<usize>,
}

fn main() {
    let cli = Cli::parse();
    let file = File::open(cli.log_filepath).unwrap();
    let games = parse_games(file).unwrap();

    if let Some(game_id) = cli.game_id {
        let found_game = games.into_iter().find(|game| game.id == game_id).unwrap();
        println!("game_{}: {}", found_game.id, found_game.match_summary(cli.death_report));
    } else {
        for game in games {
            println!("game_{}: {}", game.id, game.match_summary(cli.death_report));
        }
    }
}
