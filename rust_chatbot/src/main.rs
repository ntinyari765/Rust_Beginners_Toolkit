use std::io::{self, Write};

fn main() {
    println!("Rust Chatbot v1.0");
    println!("━━━━━━━━━━━━━━━━━━━━");
    println!("Type 'help' for commands or 'bye' to exit\n");

    // Main conversation loop
    loop {
        // Print prompt without newline
        print!("You: ");
        io::stdout().flush().unwrap();

        // Read user input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        // Trim whitespace and convert to lowercase for comparison
        let input = input.trim().to_lowercase();

        // Generate response based on input
        let response = match input.as_str() {
            "bye" | "exit" | "quit" => {
                println!("Bot: Goodbye! Thanks for chatting! 👋\n");
                break; // Exit the loop
            }
            "hello" | "hi" | "hey" => {
                "Hello there! How can I help you today? 😊"
            }
            "how are you" | "how are you?" => {
                "I'm doing great! I'm a Rust-powered chatbot, so I'm always in good shape! 🦀"
            }
            "what is rust" | "what is rust?" => {
                "Rust is a systems programming language focused on safety and performance! 🚀"
            }
            "help" | "?" => {
                "Available commands:\n  \
                 - hello: Greet me\n  \
                 - how are you: Ask how I'm doing\n  \
                 - what is rust: Learn about Rust\n  \
                 - joke: Hear a programming joke\n  \
                 - bye: Exit the chat"
            }
            "joke" => {
                "Why do Rust programmers make good friends? \
                 Because they never forget to .unwrap() their feelings! 😄"
            }
            "" => {
                "Please type something! I'm here to chat. 💬"
            }
            _ => {
                "I'm not sure how to respond to that. Type 'help' to see what I can do! 🤔"
            }
        };

        // Print bot response
        println!("Bot: {}\n", response);
    }
}
