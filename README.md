# Bracket Parser

A Rust library that uses tree-sitter to parse text and determine whether positions in the text are inside or outside brackets.

## Features

- Detects positions inside or outside brackets (parentheses, square brackets, curly braces)
- Handles nested brackets correctly
- Simple API for checking the state at the end of a string or at any position
- Built on the robust tree-sitter parsing library

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
bracket-parser = "0.1.0"
```

## Usage

```rust
use bracket_parser::{BracketParser, BracketState};

fn main() {
    // Create a new parser instance
    let mut parser = BracketParser::new().expect("Failed to initialize parser");

    // Check if the cursor would be inside brackets at the end of this string
    let code = "function call(param";
    let state = parser.get_final_state(code);

    match state {
        BracketState::Inside => println!("Cursor is inside brackets"),
        BracketState::Outside => println!("Cursor is outside brackets"),
    }

    // Get the state at each character position
    let states = parser.get_all_states("a(b)c");
    for (i, (ch, state)) in "a(b)c".chars().zip(states.iter()).enumerate() {
        println!("Character '{}' at position {} is {:?}", ch, i, state);
    }
}
```

## How It Works

The library uses tree-sitter to parse the input text into a syntax tree that recognizes bracketed expressions. It then traverses the tree to determine if a given position is inside or outside brackets.

## License

MIT
