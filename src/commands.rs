use adaptive_engine::AdaptiveEngine;
use types::Board;

pub fn run_play(depth: u32) {
    let mut engine = AdaptiveEngine::new();
    let mut board = Board::new();
    let color = types::Color::White;  // Assume the player is white for simplicity

    // Example loop to play the game
    loop {
        let best_move = engine.search_best_move(&mut board, depth as i32, color);
        println!("Engine move: {:?} -> {:?}", best_move.0, best_move.1);

        // Here, you'd add the logic to handle the user's move and update the board
        // For simplicity, we end after one move
        break;
    }
}

pub fn run_analyze(fen: &str, depth: u32) {
    let mut engine = AdaptiveEngine::new();
    let board = Board::from_fen(fen).expect("Invalid FEN string");
    let color = types::Color::White;  // Assume analyzing for White

    let best_move = engine.search_best_move(&mut board.clone(), depth as i32, color);
    println!("Best move for position: {:?} -> {:?}", best_move.0, best_move.1);
}
