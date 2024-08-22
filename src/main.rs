// The main cli/entry point of the engine.

mod commands;
mod interface;

use clap::{Arg, Command, Subcommand};
use interface::run_interactive;
use commands::{run_analyze, run_play};

fn main() {
    let matches = Command::new("KishMat Chess Engine CLI")
        .version("0.1.0")
        .author("Ahmad Hamdi <contact@hamdiz.me>")
        .about("Command-line interface for interacting with KishMat engine")
        .subcommand(
            Command::new("play")
                .about("Play a game against the engine")
                .arg(
                    Arg::new("depth")
                        .short('d')
                        .long("depth")
                        .value_name("DEPTH")
                        .help("Sets the search depth")
                ),
        )
        .subcommand(
            Command::new("analyze")
                .about("Analyze a given position")
                .arg(
                    Arg::new("fen")
                        .short('f')
                        .long("fen")
                        .value_name("FEN")
                        .help("Provide the FEN string of the position to analyze")
                )
                .arg(
                    Arg::new("depth")
                        .short('d')
                        .long("depth")
                        .value_name("DEPTH")
                        .help("Sets the search depth")
                ),
        )
        .subcommand(
            Command::new("interactive")
                .about("Run the engine in interactive mode")
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("play") {
        let depth = matches.get_one::<&str>("depth").unwrap_or(&"5").parse().unwrap();
        run_play(depth);
    } else if let Some(matches) = matches.subcommand_matches("analyze") {
        let fen = *matches.get_one::<&str>("fen").expect("FEN string is required");
        let depth = matches.get_one::<&str>("depth").unwrap_or(&"5").parse().unwrap();
        run_analyze(fen, depth);
    } else if let Some(_) = matches.subcommand_matches("interactive") {
        run_interactive();
    } else {
        println!("No valid subcommand was provided. Use --help for more information.");
    }
}
