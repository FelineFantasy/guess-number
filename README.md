# 🔢 Guess the Number [![Rust Version](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)

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

## 💖 Support the Project

If you enjoy **Guess the Number** and want to help keep the project alive, you can support me here:

[![DonationAlerts](https://img.shields.io/badge/DonationAlerts-Support-blue.svg)](https://www.donationalerts.com/r/felinefantasy)

Your support helps me:
- 🦀 Keep developing Rust games
- 🐱 Add new features and improve AI
- ☕ Stay awake while coding at 4 AM

Every little bit is appreciated! ❤️

## 👤 Author
- **FelineFantasy**
- **License**: MIT
