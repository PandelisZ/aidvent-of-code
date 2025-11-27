# aidvent-of-code

A minimal Rust project managed with Cargo.

## Prerequisites
- Rust toolchain (rustup, cargo). Install from https://rustup.rs

## Build and Run
```bash
# Compile in debug mode
cargo build

# Run the binary
cargo run
```

## Lint and Typecheck
```bash
# Typecheck (no build artifacts)
cargo check

# Lint with clippy and fail on warnings
cargo clippy --all-targets --all-features -- -D warnings
```

## Tests
```bash
cargo test
```

## Format
```bash
# Check formatting
cargo fmt -- --check

# Apply formatting
cargo fmt
```

## Project Structure
```
src/
  main.rs       # entry point
```

## License
This project is licensed under the terms of the license found in the `LICENSE` file.
