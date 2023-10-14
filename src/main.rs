use std::fs::File;

use clap::Parser;
use quake_log_parser::LogParser;

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
    let mut parser = LogParser::new(file);

    if let Some(game_id) = cli.game_id {
        match parser.find(|game| game.id == game_id) {
            Some(game) =>  println!(
                "\"game_{}\": {}",
                game.id,
                game.match_summary(cli.death_report)
            ),
            None => println!("Game not found!")
        }
    } else {
        println!("[");
        for game in parser {
            println!("\"game_{}\": {},", game.id, game.match_summary(cli.death_report));
        }
        println!("]");
    }
}
