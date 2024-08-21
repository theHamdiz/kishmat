# ♟️ KishMat

Welcome to **KishMat**! This isn’t just another chess engine—it’s a clever, adaptive, and blazing-fast powerhouse designed to challenge even the sharpest minds. Whether you're looking to play a game, analyze a position, or dive deep into the intricacies of chess AI, KishMat has you covered.

## Why KishMat?

### ⚡ Speed and Efficiency
KishMat is built with performance in mind. Leveraging Rust's speed and safety, this engine can calculate deep and wide without breaking a sweat. The use of highly optimized data structures like bitboards ensures that every move is calculated with lightning speed.

### 🧠 Adaptive Intelligence
One size doesn’t fit all—especially in chess. KishMat uses an **Adaptive Search Strategy** that tailors its approach based on the position's complexity and type. Whether it’s an open position with tons of tactical possibilities or a closed, strategic battle, KishMat knows exactly how to handle it.

### 🔍 Advanced Search Techniques
KishMat employs a mix of advanced search techniques:
- **Iterative Deepening**: Ensures the best move is always ready, even with limited time.
- **Alpha-Beta Pruning**: Cuts down on unnecessary calculations to focus on the most promising lines.
- **Quiescence Search**: Handles tricky positions with lots of tactical fireworks.
- **Late Move Reductions**: Optimizes depth for less critical moves.
- **Null Move Pruning**: A clever trick to speed up search by skipping over irrelevant moves.

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
cargo run --release --bin cli
```

### Play a Game

Start a game against the engine:

```bash
cargo run --release --bin cli play --depth 5
```

### Analyze a Position

Analyze a position from a FEN string:

```bash
cargo run --release --bin cli analyze --fen "r1bqkbnr/pppppppp/n7/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1" --depth 10
```

### Interactive Mode

For an interactive session:

```bash
cargo run --release --bin cli interactive
```

## Contributing

We’re always looking to improve! Feel free to fork this repo, submit pull requests, or open issues with suggestions. Let’s make KishMat even smarter together! 💪

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

Enjoy your journey into the world of chess AI with **KishMat**! ♟️✨

---

This README showcases the abilities and unique features of **KishMat** while providing a welcoming and clear introduction to potential users and contributors.