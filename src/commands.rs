use std::io;
use std::io::Write;
use arbiter::Arbiter;
use types::{Board, GameState, Square, ChessMove};

use std::str::FromStr;


pub fn run_play(depth: u32) {
    let mut engine = Arbiter::new();
    let mut board = Board::new();
    let player_color = types::Color::White;
    let engine_color = types::Color::Black;

    loop {
        // Engine's move
        let best_move = engine.search_best_move(&mut board, depth as i32, engine_color);
        board.apply_move(best_move, player_color);
        println!("Engine move: {:?} -> {:?}", best_move.0, best_move.1);

        // Check for game over (you might need to implement this logic based on your game)
        if GameState::is_game_over(&board, player_color) {
            println!("Game over! The engine wins.");
            break;
        }

        // Display the current board state
        println!("{:?}", board);

        // User's move
        let mut user_input = String::new();
        println!("Your move (e.g., e2e4): ");
        io::stdout().flush().expect("Failed to flush stdout");

        io::stdin().read_line(&mut user_input).expect("Failed to read line");

        let user_move = parse_move(&user_input, player_color);
        match user_move {
            Some(mv) => {
                if board.is_legal_move(mv, player_color) {
                    board.apply_move(mv.clone(), player_color);
                } else {
                    println!("Illegal move! Please try again.");
                    continue;
                }
            }
            None => {
                println!("Invalid input! Please try again.");
                continue;
            }
        }

        // Check for game over
        if GameState::is_game_over(&board, player_color) {
            println!("Game over! You win.");
            break;
        }

        // Display the current board state
        println!("{:?}", board);
    }
}

fn parse_move(input: &str, color: types::Color) -> Option<ChessMove> {
    // Normalize the input: remove all non-alphanumeric characters
    let normalized_input: String = input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();

    // Ensure the normalized input has exactly 4 characters (e.g., "g3g4")
    if normalized_input.len() != 4 {
        return None;
    }

    // Extract the source and destination squares
    let (source, destination) = normalized_input.split_at(2);

    // Convert source and destination into Squares (replace String with your Square type)
    let source_square = Square::from_str(source).ok()?;
    let destination_square = Square::from_str(destination).ok()?;

    // Return the move as a tuple (source_square, destination_square)
    Some((source_square, destination_square))
}

pub fn run_analyze(fen: &str, depth: u32) {
    let mut engine = Arbiter::new();
    let board = Board::from_fen(fen).expect("Invalid FEN string");
    let color = types::Color::White;  // Assume analyzing for White

    let best_move = engine.search_best_move(&mut board.clone(), depth as i32, color);
    println!("Best move for position: {:?} -> {:?}", best_move.0, best_move.1);
}
