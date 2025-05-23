# Key-Value Store

A simple in-memory key-value store implemented in Rust using `HashMap<String, String>`.

## Features

- Insert, retrieve, and delete string key-value pairs
- Safe and idiomatic Rust code
- Linting with `clippy`
- Custom developer workflow using `cargo-make`

## Getting Started

### Prerequisites

Install Rust and the `cargo-make` tool:

    cargo install cargo-make

### Run the App with Clippy Checks

To run the app and ensure code quality checks pass:

    cargo make check_and_run

This will:
1. Run `cargo clippy` with all recommended lints.
2. Run the application.

### Manual Commands

If you prefer running steps individually:

    cargo clippy -- -W clippy::all -W clippy::pedantic -W clippy::nursery
    cargo run

## Project Structure

- `src/`
  - `main.rs`: Application entry point
  - `persistence/key_value_store.rs`: The `KeyValueStore` implementation

## Tasks
- [ ] Add tests
- [ ] Add logging
- [ ] Add persistence via append only log written to disk
- [ ] Add expriation to keys via time to live
- [ ] Add CLI interface
- [ ] Add thread-safe access
- [ ] Add HTTP interface
- [ ] Add LRU cache capability