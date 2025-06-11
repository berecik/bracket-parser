//! Examples demonstrating how to use the BracketParser library.

use bracket_parser::{BracketParser, BracketState};

fn main() {
    // Example 1: Basic usage with sample code
    println!("Example 1: Basic usage with sample code\n");
    basic_usage_example();

    // Example 2: Check final states of various inputs
    println!("\nExample 2: Check final states of various inputs\n");
    final_state_examples();

    // Example 3: Real-time typing simulation
    println!("\nExample 3: Real-time typing simulation\n");
    typing_simulation_example();

    // Example 4: Analyzing code with nested brackets
    println!("\nExample 4: Analyzing code with nested brackets\n");
    nested_brackets_example();
}

// Example 1: Basic usage with detailed character state analysis
fn basic_usage_example() {
    // Initialize the parser
    let mut parser = BracketParser::new()
        .expect("Failed to initialize bracket parser");

    // Sample code to analyze
    let code = "function call(param1, param2)";

    // Get states for all character positions
    let states = parser.get_all_states(code);

    // Display results in a table format
    println!("Analyzing: \"{}\"\n", code);
    println!("{:<5} | {:<10} | {:<10}", "Pos", "Character", "State");
    println!("{}", "-".repeat(30));

    for (i, ch) in code.char_indices() {
        println!("{:<5} | {:<10} | {:<10?}", 
                i, 
                format!("'{}'", ch), 
                states[i]);
    }
}

// Example 2: Checking final states of different inputs
fn final_state_examples() {
    let mut parser = BracketParser::new().expect("Failed to initialize parser");

    let examples = [
        "Simple text without brackets",
        "Text with (unclosed bracket",
        "Text with (closed bracket)",
        "Nested [brackets (inside) each other]",
        "Multiple types {curly and [square]}",
        "Complex (example [with {nested} brackets])",
        "Unbalanced ([brackets)",
        "Markdown link: [title](https://example.com)",
    ];

    println!("{:<40} | {:<10}", "Input", "Final State");
    println!("{}", "-".repeat(55));

    for example in examples {
        let state = parser.get_final_state(example);
        println!("{:<40} | {:<10?}", 
                if example.len() > 38 { 
                    format!("{:.35}...", example) 
                } else { 
                    example.to_string() 
                }, 
                state);
    }
}

// Example 3: Simulating real-time typing with state updates
fn typing_simulation_example() {
    let mut parser = BracketParser::new().expect("Failed to initialize parser");

    // Simulate typing a function call character by character
    let typing_sequence = "calculate(2 + 3)";
    let mut current_text = String::new();

    println!("{:<20} | {:<10}", "Current Text", "State");
    println!("{}", "-".repeat(35));

    for ch in typing_sequence.chars() {
        current_text.push(ch);
        let state = parser.get_final_state(&current_text);

        println!("{:<20} | {:<10?}", current_text, state);
    }
}

// Example 4: Analyzing a more complex nested structure
fn nested_brackets_example() {
    let mut parser = BracketParser::new().expect("Failed to initialize parser");

    let nested_code = "if (condition) { doSomething(); while [i < items.length] {process(items[i])} }";
    let states = parser.get_all_states(nested_code);

    // Display the code with bracket state visualization
    println!("Code with bracket depth visualization:\n");
    println!("{}", nested_code);

    // Create a visual representation of bracket depth
    let state_markers: String = nested_code
        .char_indices()
        .map(|(i, ch)| match (ch, states[i]) {
            ('(', _) | ('[', _) | ('{', _) => '(',
            (')', _) | (']', _) | ('}', _) => ')',
            (_, BracketState::Inside) => '_',
            (_, BracketState::Outside) => ' ',
        })
        .collect();

    println!("{}", state_markers);

    // Count characters by state
    let inside_count = states.iter().filter(|&&s| s == BracketState::Inside).count();
    let outside_count = states.iter().filter(|&&s| s == BracketState::Outside).count();

    println!("\nStatistics:");
    println!("- Total characters: {}", nested_code.len());
    println!("- Characters inside brackets: {} ({}%)", 
             inside_count, 
             (inside_count as f32 / nested_code.len() as f32 * 100.0).round());
    println!("- Characters outside brackets: {} ({}%)", 
             outside_count, 
             (outside_count as f32 / nested_code.len() as f32 * 100.0).round());
}
