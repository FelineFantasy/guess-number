# 🔢 Guess the Number

A game where the computer guesses a number you thought of from 1 to 100 in 7 attempts (or less). Implemented using binary search.

**🦀 Rust version** – fast, safe, and compiled

## 🎮 How to Play

1. Run the program: `cargo run`
2. Think of a number from 1 to 100
3. Answer the computer's questions:
   - `>` — if your number is greater
   - `<` — if your number is less
   - `=` — if the computer guessed it right
   - `exit`, `quit`, `q` — exit the game

## 🧠 Features

- Robust to invalid input (letters, empty lines)
- Attempt counting
- Cheating protection (detects contradictory answers)
- **Exit command** – you can escape the interrogation 😼

## 🛠️ Installation

```bash
git clone https://github.com/FelineFantasy/guess-number.git
cd guess-number
cargo build --release
./target/release/guess-number
```

## 👤 Author
- **FelineFantasy**
- **License**: MIT
