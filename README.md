# ♟️ KishMat

Welcome to **KishMat**! This isn’t just another chess engine—it’s a clever, adaptive, and blazing-fast powerhouse designed to challenge even the sharpest minds. Whether you're looking to play a game, analyze a position, or dive deep into the intricacies of chess AI, KishMat has you covered.

## Why KishMat?

### ⚡ Speed and Efficiency
KishMat is built with performance in mind. Leveraging Rust's speed and safety, this engine can calculate deep and wide without breaking a sweat. The use of highly optimized data structures like bitboards ensures that every move is calculated with lightning speed.

### 🧠 Adaptive Intelligence
One size doesn’t fit all—especially in chess. KishMat uses an **Adaptive Search Strategy** that tailors its approach based on the position's complexity and type. Whether it’s an open position with tons of tactical possibilities or a closed, strategic battle, KishMat knows exactly how to handle it.

### 🔍 Advanced Search Techniques
KishMat employs a mix of advanced search techniques:
- **Alpha-Beta Pruning**: Prunes unnecessary branches in the search tree, allowing the engine to focus on the most promising lines.
- **Iterative Deepening**: Gradually increases the search depth, ensuring that the best move found so far is always available, even with limited time.
- **Late Move Reductions**: Reduces the search depth for less promising moves, allowing more focus on likely candidates.
- **NegaMax**: A variant of the Minimax algorithm that simplifies the code by assuming one player is trying to maximize and the other is trying to minimize the same score.
- **Null Move Pruning**: Speeds up the search by temporarily skipping the opponent's move, helping to prune irrelevant branches.
- **Quiescence Search**: Extends the search at "noisy" positions to avoid the horizon effect, focusing on positions with tactical threats like captures or checks.
- **Transposition Table**: Caches previously searched positions to avoid redundant calculations and speed up future searches.

### 🎯 Precision Evaluation
Our evaluation function isn’t just about material counts. It considers piece activity, king safety, pawn structure, and more. KishMat knows that a well-placed knight can be more valuable than a trapped rook.

### 🧩 Modular Design
KishMat is built as a collection of Rust crates, making it easy to extend, modify, and integrate with other projects. Want to add your own twist? Jump in!

### 🌍 Universal Communication
KishMat speaks your language, whether you’re using UCI, XBoard, or our custom CLI. It’s ready to integrate into your favorite chess GUI or work directly from the command line.

## Features at a Glance

- **CLI Interface**: Play games, analyze positions, or run in interactive mode right from your terminal.
- **Adaptive Search**: Dynamically adjusts its strategy based on the position.
- **Bitboard-Powered**: Efficiently represents and manipulates board states.
- **Advanced Evaluation**: Goes beyond material count to assess the true strength of positions.
- **Extensible Architecture**: Easy to modify and extend, thanks to its modular design.

## Getting Started

### Installation

Clone the repository:

```bash
git clone https://github.com/yourusername/kishmat.git
cd kishmat
```

Build the project:

```bash
cargo build --release
```

Run the CLI:

```bash
cargo run --release
```

### Play a Game

Start a game against the engine:

```bash
cargo run --release play --depth 5
```

### Analyze a Position

Analyze a position from a FEN string:

```bash
cargo run --release analyze --fen "r1bqkbnr/pppppppp/n7/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1" --depth 10
```

### Interactive Mode

For an interactive session:

```bash
cargo run --release interactive
```

## Contributing

We’re always looking to improve! Feel free to fork this repo, submit pull requests, or open issues with suggestions. Let’s make KishMat even smarter together! 💪

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

Enjoy your journey into the world of chess AI with **KishMat**! ♟️✨

---