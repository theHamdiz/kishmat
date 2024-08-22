use arbiter::Arbiter;
use types::{Board, Square, Color};
use std::io::{self, Write};

pub fn run_interactive() {
    let mut engine = Arbiter::new();
    let mut board = Board::new();
    let color = Color::White;  // Assume the player is white for simplicity

    loop {
        print!("Enter your move (e.g., e2e4): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();

        if trimmed.len() == 4 {
            let from_square = parse_square(&trimmed[0..2]).expect("Invalid from square");
            let to_square = parse_square(&trimmed[2..4]).expect("Invalid to square");

            // Apply the player's move
            let (piece, color) = board.get_piece_at_square(from_square).expect("Could not get piece at a given square");
            board.make_move(from_square, to_square, piece, color);

            // Engine's response
            let best_move = engine.search_best_move(&mut board, 5, color.opponent());
            println!("Engine move: {:?} -> {:?}", best_move.0, best_move.1);

            board.make_move(best_move.0, best_move.1, piece, color.opponent());
        } else {
            println!("Invalid move format. Please use the format 'e2e4'.");
        }
    }
}

fn parse_square(input: &str) -> Option<Square> {
    match input {
        "a1" => Some(Square::A1),
        "b1" => Some(Square::B1),
        "c1" => Some(Square::C1),
        "d1" => Some(Square::D1),
        "e1" => Some(Square::E1),
        "f1" => Some(Square::F1),
        "g1" => Some(Square::G1),
        "h1" => Some(Square::H1),
        "a2" => Some(Square::A2),
        "b2" => Some(Square::B2),
        "c2" => Some(Square::C2),
        "d2" => Some(Square::D2),
        "e2" => Some(Square::E2),
        "f2" => Some(Square::F2),
        "g2" => Some(Square::G2),
        "h2" => Some(Square::H2),
        "a3" => Some(Square::A3),
        "b3" => Some(Square::B3),
        "c3" => Some(Square::C3),
        "d3" => Some(Square::D3),
        "e3" => Some(Square::E3),
        "f3" => Some(Square::F3),
        "g3" => Some(Square::G3),
        "h3" => Some(Square::H3),
        "a4" => Some(Square::A4),
        "b4" => Some(Square::B4),
        "c4" => Some(Square::C4),
        "d4" => Some(Square::D4),
        "e4" => Some(Square::E4),
        "f4" => Some(Square::F4),
        "g4" => Some(Square::G4),
        "h4" => Some(Square::H4),
        "a5" => Some(Square::A5),
        "b5" => Some(Square::B5),
        "c5" => Some(Square::C5),
        "d5" => Some(Square::D5),
        "e5" => Some(Square::E5),
        "f5" => Some(Square::F5),
        "g5" => Some(Square::G5),
        "h5" => Some(Square::H5),
        "a6" => Some(Square::A6),
        "b6" => Some(Square::B6),
        "c6" => Some(Square::C6),
        "d6" => Some(Square::D6),
        "e6" => Some(Square::E6),
        "f6" => Some(Square::F6),
        "g6" => Some(Square::G6),
        "h6" => Some(Square::H6),
        "a7" => Some(Square::A7),
        "b7" => Some(Square::B7),
        "c7" => Some(Square::C7),
        "d7" => Some(Square::D7),
        "e7" => Some(Square::E7),
        "f7" => Some(Square::F7),
        "g7" => Some(Square::G7),
        "h7" => Some(Square::H7),
        "a8" => Some(Square::A8),
        "b8" => Some(Square::B8),
        "c8" => Some(Square::C8),
        "d8" => Some(Square::D8),
        "e8" => Some(Square::E8),
        "f8" => Some(Square::F8),
        "g8" => Some(Square::G8),
        "h8" => Some(Square::H8),
        _ => None,
    }
}
