# Rust Command-Line Chatbot 🤖

A simple interactive chatbot built with Rust to demonstrate core language concepts.

## Features

- 💬 Interactive command-line interface
- 🎯 Pattern matching for intelligent responses
- 🔄 Continuous conversation loop
- 🛠️ Built-in help command
- 👋 Graceful exit handling

## Quick Start

```bash
# Make sure you're in the rust-chatbot directory
cd rust-chatbot

# Run the chatbot
cargo run
```

## Available Commands

Once the chatbot is running, try these commands:

- `hello` - Greet the bot
- `how are you` - Ask how the bot is doing
- `what is rust` - Learn about Rust
- `joke` - Hear a programming joke
- `help` - See all available commands
- `bye` - Exit the chat

## Example Conversation

```
Rust Chatbot v1.0
━━━━━━━━━━━━━━━━━━━━
Type 'help' for commands or 'bye' to exit

You: hello
Bot: Hello there! How can I help you today? 😊

You: what is rust
Bot: Rust is a systems programming language focused on safety and performance! 🚀

You: joke
Bot: Why do Rust programmers make good friends? Because they never forget to .unwrap() their feelings! 😄

You: bye
Bot: Goodbye! Thanks for chatting! 👋
```

## Building & Running

### Development Mode (Fast compilation)
```bash
cargo run
```

### Release Mode (Optimized)
```bash
cargo build --release
./target/release/rust-chatbot
```

## Project Structure

```
rust-chatbot/
├── Cargo.toml       # Project configuration and dependencies
├── Cargo.lock       # Dependency lock file
├── src/
│   └── main.rs      # Main chatbot source code
└── README.md        # This file
```

## Requirements

- Rust 1.70.0 or higher
- Cargo (comes with Rust)

## Learning Resources

For the complete learning guide, including:
- Detailed setup instructions
- AI prompts used during development
- Common errors and solutions
- Learning reflections

See the main [GUIDE.md](../GUIDE.md) in the repository root.

## Extending the Chatbot

Want to add more features? Try:

1. **Add new responses** - Extend the `match` statement in `main.rs`
2. **Track conversation count** - Add a counter variable
3. **Save chat history** - Write conversations to a file
4. **Add random responses** - Use the `rand` crate for variety

## Troubleshooting

### "cargo: command not found"
Make sure Rust is installed and in your PATH:
```bash
source $HOME/.cargo/env
```

### Build takes a long time
This is normal for the first build. Subsequent builds are much faster!

## License

MIT