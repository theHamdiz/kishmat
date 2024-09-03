// The main cli/entry point of the engine.

mod commands;
mod interface;

use std::io;
use clap::{Arg, ArgMatches, Command};
use interface::run_interactive;
use commands::{run_analyze, run_play};

// fn main() {
//     let matches = Command::new("KishMat Chess Engine CLI")
//         .version("0.1.0")
//         .author("Ahmad Hamdi <contact@hamdiz.me>")
//         .about("Command-line interface for interacting with KishMat engine")
//         .subcommand(
//             Command::new("play")
//                 .about("Play a game against the engine")
//                 .arg(
//                     Arg::new("depth")
//                         .short('d')
//                         .long("depth")
//                         .value_name("DEPTH")
//                         .help("Sets the search depth"),
//                 ),
//         )
//         .subcommand(
//             Command::new("analyze")
//                 .about("Analyze a given position")
//                 .arg(
//                     Arg::new("fen")
//                         .short('f')
//                         .long("fen")
//                         .value_name("FEN")
//                         .help("Provide the FEN string of the position to analyze"),
//                 )
//                 .arg(
//                     Arg::new("depth")
//                         .short('d')
//                         .long("depth")
//                         .value_name("DEPTH")
//                         .help("Sets the search depth"),
//                 ),
//         )
//         .subcommand(
//             Command::new("interactive")
//                 .about("Run the engine in interactive mode"),
//         )
//         .get_matches();
// 
//     if let Some(matches) = matches.subcommand_matches("play") {
//         let depth = matches.get_one::<&str>("depth").unwrap_or(&"5").parse().unwrap();
//         run_play(depth);
//     } else if let Some(matches) = matches.subcommand_matches("analyze") {
//         let fen = *matches.get_one::<&str>("fen").expect("FEN string is required");
//         let depth = matches.get_one::<&str>("depth").unwrap_or(&"5").parse().unwrap();
//         run_analyze(fen, depth);
//     } else if let Some(_) = matches.subcommand_matches("interactive") {
//         run_interactive();
//     } else {
//         println!("Welcome to KishMat Chess Engine, By Ahmad Hamdi, Egypt!");
//         println!("You can choose to:");
//         println!("1. Play a game against the engine (type 'play')");
//         println!("2. Analyze a position (type 'analyze')");
//         println!("3. Run the engine in interactive mode (type 'interactive')");
//         println!("4. Exit the program (type 'exit')");
//         
//         loop {
//             // Wait for user input
//             let mut input = String::new();
//             io::stdin().read_line(&mut input).expect("Failed to read line");
//             let input = input.trim().to_lowercase();
// 
//             match input.as_str() {
//                 "play" => {
//                     println!("Enter the desired depth (or press Enter to use the default):");
//                     let mut depth_input = String::new();
//                     io::stdin().read_line(&mut depth_input).expect("Failed to read line");
//                     let depth = depth_input.trim().parse().unwrap_or(5);
//                     run_play(depth);
//                     break;
//                 }
//                 "analyze" => {
//                     println!("Enter the FEN string:");
//                     let mut fen = String::new();
//                     io::stdin().read_line(&mut fen).expect("Failed to read line");
//                     println!("Enter the desired depth (or press Enter to use the default):");
//                     let mut depth_input = String::new();
//                     io::stdin().read_line(&mut depth_input).expect("Failed to read line");
//                     let depth = depth_input.trim().parse().unwrap_or(5);
//                     run_analyze(&fen.trim(), depth);
//                     break;
//                 }
//                 "interactive" => {
//                     run_interactive();
//                     break;
//                 }
//                 "exit" => {
//                     println!("Goodbye!");
//                     break;
//                 }
//                 _ => {
//                     println!("Invalid option. Please type 'play', 'analyze', 'interactive', or 'exit'.");
//                 }
//             }
//         }
//     }
// }

use rand::Rng;
use search::OpeningBook;
// To handle random color selection

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
                        .help("Sets the search depth"),
                )
                .arg(
                    Arg::new("color")
                        .short('c')
                        .long("color")
                        .value_name("COLOR")
                        .help("Choose the color you want to play as (w for White, b for Black, r for Random)")
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
                        .help("Provide the FEN string of the position to analyze"),
                )
                .arg(
                    Arg::new("depth")
                        .short('d')
                        .long("depth")
                        .value_name("DEPTH")
                        .help("Sets the search depth"),
                ),
        )
        .subcommand(
            Command::new("interactive")
                .about("Run the engine in interactive mode"),
        )
        .get_matches();

        repl(matches);
    }

fn repl(matches: ArgMatches) {
    if let Some(matches) = matches.subcommand_matches("play") {
        let depth = matches.get_one::<&str>("depth").unwrap_or(&"5").parse().unwrap();
        let color_str = matches.get_one::<&str>("color").unwrap_or(&"w");
        let player_color = match *color_str {
            "w" => types::Color::White,
            "b" => types::Color::Black,
            "r" => {
                let mut rng = rand::thread_rng();
                if rng.gen_bool(0.5) {
                    types::Color::White
                } else {
                    types::Color::Black
                }
            }
            _ => types::Color::White, // Default to White if invalid input
        };

          println!("Player chose to play as {:?}", player_color);

          let mut book = OpeningBook::new("assets\\books\\Perfect2023.bin");
          book.load_from_file().expect("Could not load book from file");
          run_play(depth, player_color, &book);
    } else if let Some(matches) = matches.subcommand_matches("analyze") {
        let fen = *matches.get_one::<&str>("fen").expect("FEN string is required");
        let depth = matches.get_one::<&str>("depth").unwrap_or(&"5").parse().unwrap();
        run_analyze(fen, depth);
    } else if let Some(_) = matches.subcommand_matches("interactive") {
        run_interactive();
        } else {
        println!("Welcome to KishMat Chess Engine, By Ahmad Hamdi, Egypt!");
        println!("You can choose to:");
        println!("1. Play a game against the engine (type 'play')");
        println!("2. Analyze a position (type 'analyze')");
        println!("3. Run the engine in interactive mode (type 'interactive')");
        println!("4. Exit the program (type 'exit')");
        
        loop {
            // Wait for user input
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let input = input.trim().to_lowercase();

            match input.as_str() {
                "play" => {
                    println!("Enter the desired depth (or press Enter to use the default):");
                    let mut depth_input = String::new();
                    let mut color_input = String::new();
                    io::stdin().read_line(&mut depth_input).expect("Failed to read line");
                    io::stdin().read_line(&mut color_input).expect("Failed to read line");
                    let depth = depth_input.trim().parse().unwrap_or(5);
                    let color = depth_input.trim();
                    let player_color = match color {
                        "w" => types::Color::White,
                        "b" => types::Color::Black,
                        "r" => {
                            let mut rng = rand::thread_rng();
                            if rng.gen_bool(0.5) {
                                types::Color::White
                            } else {
                                types::Color::Black
                            }
                        }
                        _ => types::Color::White, // Default to White if invalid input
                    };
                    let mut book = OpeningBook::new("assets\\books\\Perfect_2010.abk");
                    book.load_from_file().expect("Could not load book from file");
                    run_play(depth, player_color, &book);
                    break;
                }
                "analyze" => {
                    println!("Enter the FEN string:");
                    let mut fen = String::new();
                    io::stdin().read_line(&mut fen).expect("Failed to read line");
                    println!("Enter the desired depth (or press Enter to use the default):");
                    let mut depth_input = String::new();
                    io::stdin().read_line(&mut depth_input).expect("Failed to read line");
                    let depth = depth_input.trim().parse().unwrap_or(5);
                    run_analyze(&fen.trim(), depth);
                    break;
                }
                "interactive" => {
                    run_interactive();
                    break;
                }
                "exit" => {
                    println!("Goodbye!");
                    break;
                }
                _ => {
                    println!("Invalid option. Please type 'play', 'analyze', 'interactive', or 'exit'.");
                }
            }
        }
    }
}



