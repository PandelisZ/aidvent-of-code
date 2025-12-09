# Rust TUI Snake Game

A simple Snake game running in your terminal, built with ratatui and crossterm.

- Smooth keyboard input (arrows/WASD)
- Score display and game-over prompt
- Restart in-place without quitting
- Clean terminal restore on exit/panic

## Quick start

Requirements:
- Rust toolchain (stable). Install via https://rustup.rs if needed.

Clone and run:

```bash
cargo run
```

Controls:
- Move: Arrow keys or WASD
- Quit: q or Esc (Ctrl+C also exits)
- Restart (after game over): r

## Development

Typecheck:
```bash
cargo check
```

Lint (treat warnings as errors):
```bash
cargo clippy -- -D warnings
```

Format check:
```bash
cargo fmt -- --check
```

Run tests (none yet, placeholder):
```bash
cargo test
```

## Project layout

- `src/main.rs`: game loop, input handling, drawing, and state logic.
- `Cargo.toml`: dependencies and package metadata.

## Configuration

Default grid size and tick rate are defined in code (GameConfig::default). If you want different sizes or speed, tweak values there:
- grid_w: width in cells
- grid_h: height in cells
- tick: step duration in milliseconds

## Troubleshooting

- If your terminal looks weird after a crash, restore it with one of:
  - `reset`
  - `stty sane` (POSIX)
- Windows users should run in a recent Windows Terminal or PowerShell. crossterm supports Windows natively.

## Dependencies

- ratatui = "0.26" — Terminal UI library
- crossterm = "0.27" — Cross-platform terminal handling
- rand = "0.8" — Food placement

## License

This project is licensed under the terms of the LICENSE file in this repository.

## Acknowledgements

Thanks to the ratatui and crossterm communities for excellent terminal tooling.
