use std::io;
use std::io::Write;
use arbiter::Arbiter;
use types::{Board, GameState, Square, ChessMove, Color, Zobrist};
use search::OpeningBook;
use std::str::FromStr;

/// TODO: Add the ability to signal to the engine the color the player wants to play with, either w,b or r for random.
/// TODO: Add debug statements to highlight the issues to me.

// #[inline(always)]
// pub fn run_play(depth: u32, player_color: Color) {
//     let mut engine = Arbiter::new();
//     let mut board = Board::new();
//     let engine_color = player_color.opponent();
// 
//     // If the player is White, ask for their move first
//     if player_color == Color::White {
//         // User's move
//         get_input(&mut board, player_color);
// 
//         // Check for game over
//         if GameState::is_game_over(&board, player_color) {
//             println!("Game over! You win.");
//             return;
//         }
//     }
// 
//     // Main game loop
//     loop {
//         // Engine's move
//         let best_move = engine.search_best_move(&mut board, depth as i32, engine_color);
//         // Before applying the move
//         println!("Attempting to apply move: {:?} -> {:?}", best_move, engine_color);
//         board.apply_move(best_move, engine_color);
//         println!("Engine move: {:?} -> {:?}", best_move.0, best_move.1);
// 
//         // Check for game over after the engine's move
//         if GameState::is_game_over(&board, engine_color) {
//             println!("Game over! The engine wins.");
//             break;
//         }
// 
//         // User's move
//         get_input(&mut board, player_color);
// 
//         // Check for game over after the player's move
//         if GameState::is_game_over(&board, player_color) {
//             println!("Game over! You win.");
//             break;
//         }
//     }
// }
// 
// #[inline(always)]
// pub fn get_input(board: &mut Board, player_color: Color) {
//     loop {
//         let mut user_input = String::new();
//         println!("Your move (e.g., e2e4): ");
//         io::stdout().flush().expect("Failed to flush stdout");
// 
//         io::stdin().read_line(&mut user_input).expect("Failed to read line");
// 
//         let user_move = parse_move(&user_input.trim());
//         match user_move {
//             Some(mv) => {
//                 println!("Debug: Attempting move {:?}", mv);
//                 if board.is_legal_move(mv, player_color) {
//                     board.apply_move(mv.clone(), player_color);
//                     println!("Debug: Move {:?} applied", mv);
//                     break; // Exit the loop if the move is legal
//                 } else {
//                     println!("Illegal move! Please try again.");
//                 }
//             }
//             None => {
//                 println!("Invalid input! Please try again.");
//             }
//         }
//     }
// }


#[inline(always)]
pub fn run_play(depth: u32, player_color: Color, opening_book: &OpeningBook) {
    let mut engine = Arbiter::new();
    let mut board = Board::new();
    let engine_color = player_color.opponent();
    let mut in_opening_phase = true;

    // If the player is White, ask for their move first
    if player_color == Color::White {
        // User's move
        get_input(&mut board, player_color);

        // Check for game over
        if GameState::is_game_over(&board, player_color) {
            println!("Game over! You win.");
            return;
        }
    }

    // Main game loop
    loop {
        let zobrist = Zobrist::default();
        let position_hash = board.compute_zobrist_hash(&zobrist);
       
        if in_opening_phase {
            if let Some((book_move, opening_name)) = opening_book.get_move(position_hash) {
                println!("Opening: {}", opening_name);
                board.apply_move(OpeningBook::polyglot_move_to_squares(book_move), engine_color);
                println!("Engine move (from book): {:?} -> {:?}", book_move, engine_color);
            } else {
                println!("No opening found in book for the current position.");
                println!("Opening phase complete");
                in_opening_phase = false;
            }
        }

        if !in_opening_phase {
            // Engine's move
            let best_move = engine.search_best_move(&mut board, depth as i32, engine_color);
            board.apply_move(best_move, engine_color);
            println!("Engine move: {:?} -> {:?}", best_move.0, best_move.1);
        }

        // Check for game over after the engine's move
        if GameState::is_game_over(&board, engine_color) {
            println!("Game over! The engine wins.");
            break;
        }

        // User's move
        get_input(&mut board, player_color);

        // Check for game over after the player's move
        if GameState::is_game_over(&board, player_color) {
            println!("Game over! You win.");
            break;
        }
    }
}


#[inline(always)]
pub fn get_input(board: &mut Board, player_color: Color) {
    loop {
        let mut user_input = String::new();
        println!("Your move (e.g., e2e4): ");
        io::stdout().flush().expect("Failed to flush stdout");

        io::stdin().read_line(&mut user_input).expect("Failed to read line");

        let user_move = parse_move(&user_input.trim());
        match user_move {
            Some(mv) => {
                println!("Debug: Attempting move {:?}", mv);
                if board.is_legal_move(mv, player_color) {
                    board.apply_move(mv.clone(), player_color);
                    println!("Debug: Move {:?} applied", mv);
                    break; // Exit the loop if the move is legal
                } else {
                    println!("Illegal move! Please try again.");
                }
            }
            None => {
                println!("Invalid input! Please try again.");
            }
        }
    }
}

#[inline(always)]
fn parse_move(input: &str) -> Option<ChessMove> {
    let normalized_input: String = input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_lowercase();

    if normalized_input.len() != 4 {
        return None;
    }

    let (source, destination) = normalized_input.split_at(2);

    let source_square = Square::from_str(source).ok()?;
    let destination_square = Square::from_str(destination).ok()?;

    Some((source_square, destination_square))
}


#[inline(always)]
pub fn run_analyze(fen: &str, depth: u32) {
    let mut engine = Arbiter::new();
    let board = Board::from_fen(fen).expect("Invalid FEN string");
    let color = Color::White;  // Assume analyzing for White

    let best_move = engine.search_best_move(&mut board.clone(), depth as i32, color);
    println!("Best move for position: {:?} -> {:?}", best_move.0, best_move.1);
}
