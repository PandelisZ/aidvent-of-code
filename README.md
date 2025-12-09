# Rust TUI Snake Game

A simple Snake game running in your terminal, built with ratatui and crossterm.

- Smooth keyboard input (arrows/WASD)
- Score display and game-over prompt
- Restart in-place without quitting
- Clean terminal restore on exit/panic

## Quick start

Requirements:
- Rust toolchain (stable). Install via https://rustup.rs if needed.

Run in debug mode:
```bash
cargo run
```

Build and run in release mode (faster):
```bash
time cargo build --release
./target/release/aidvent-of-code
```

Controls:
- Move: Arrow keys or WASD
- Quit: q or Esc (Ctrl+C also exits)
- Restart (after game over): r

## Installation options

- As a developer (recommended): clone the repo and use `cargo run`.
- As a binary: build with `cargo build --release` and copy the binary from `target/release/`.

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

Tests (none yet):
```bash
cargo test
```

Run with logging (example using RUST_LOG; not required by the game):
```bash
RUST_LOG=info cargo run
```

## Project layout

- `src/main.rs`: game loop, input handling, drawing, and state logic.
- `Cargo.toml`: dependencies and package metadata.

## Configuration

Defaults are defined in code (`GameConfig::default`). To change gameplay:
- grid_w: width in cells
- grid_h: height in cells
- tick: step duration in milliseconds

If you want runtime configuration, consider adding CLI flags (e.g. with `clap`) for these fields.

## Key bindings

- Arrows / WASD: move
- r: restart (after game over)
- q or Esc: quit
- Ctrl+C: quit

## Cross‑platform notes

- Linux/macOS: works in most POSIX terminals (xterm, GNOME Terminal, iTerm2, etc.).
- Windows: works in Windows Terminal and recent PowerShell/Command Prompt via crossterm.

## Performance tips

- Use `cargo run --release` or `cargo build --release` for smoother movement on slower machines.
- Larger grids require more rendering work each frame; reduce `grid_w`/`grid_h` for lightweight terminals.

## How it works

- Game loop: polls input with `crossterm::event::poll`, advances state on a fixed tick (`GameConfig::tick`), and redraws every frame with ratatui.
- Data structures:
  - `Pos` (x, y) grid coordinates
  - `Dir` (Up/Down/Left/Right) with a `delta()` helper
  - `SnakeGame` state: deque for snake body, current dir, food position, score, flags
- Rendering: builds a grid of `Line`/`Span` where
  - head = `@` (yellow), body = `o` (green), food = `*` (red), empty = space
  - a bordered playfield is centered inside the terminal
- Terminal lifecycle: enables raw mode, enters the alternate screen, hides the cursor; a panic hook ensures restoration on unexpected errors.

## Customization

- Symbols and colors: update the characters (`@`, `o`, `*`) and `Style` colors in the `draw()` function.
- Size and speed: tweak `GameConfig::default()` (grid width/height and tick duration).
- Themes: define a small theme struct with colors and pass it to drawing helpers.
- Wrap-around or walls: adjust the collision logic in `step()`.

## Packaging and cross‑compiling

Build a release binary for your platform:
```bash
cargo build --release
```

Linux static (musl) example:
```bash
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
# binary at target/x86_64-unknown-linux-musl/release/aidvent-of-code
```

Using cross for easier cross‑compiles:
```bash
cargo install cross
cross build --release --target x86_64-unknown-linux-musl
```

Windows (MSVC toolchain):
```bash
# On Windows with MSVC installed
cargo build --release --target x86_64-pc-windows-msvc
```

macOS (Intel/Apple Silicon):
```bash
# Build natively for your host architecture
cargo build --release
# Or build for a specific target, e.g.
rustup target add aarch64-apple-darwin x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
```

Distribute by zipping the release binary and including LICENSE/README.

## FAQ

- The terminal didn’t restore after a crash.
  - Run `reset` (POSIX) or `stty sane`. Reopen the terminal if needed.
- Input is laggy.
  - Use a release build and avoid very large grids; reduce tick duration moderately.
- Can I add a pause key?
  - Yes: add a boolean pause flag toggled by a key (e.g. `p`) and early return from `step()` if paused.
- Mouse support?
  - crossterm supports mouse events; they aren’t used here, but you can extend input handling accordingly.

## Known issues / notes

- Very small terminal sizes may clip the playfield or header.
- Some fonts/terminals render wide glyphs differently; stick to ASCII for predictable layout.
- This repository may contain Node-related files (package.json, node_modules) which are not required for building/running the Rust game. You can ignore them when working with Cargo.

## Roadmap / ideas

- Pause toggle
- Persistent high score
- CLI options for grid size and speed
- Different difficulty levels
- Walls / obstacles and wrap‑around mode
- Color themes
- Basic unit tests for movement and collision logic

## Contributing

- Use `cargo fmt` before committing.
- Run `cargo clippy -- -D warnings` to keep the code tidy.
- Open a PR with a concise description and, if possible, a short demo (gif/asciinema).

## Troubleshooting

- If your terminal looks weird after a crash, restore it with one of:
  - `reset`
  - `stty sane` (POSIX)
- If input seems unresponsive in a multiplexed session (tmux/screen), ensure the pane is focused and try disabling conflicting keybindings.
- Windows users should run in a recent Windows Terminal or PowerShell; crossterm supports Windows natively.

## Screenshots / demo

You can capture a short session with asciinema:
```bash
asciinema rec
# play the game, then exit
asciinema stop
```

## Dependencies

- ratatui = "0.26" — Terminal UI library
- crossterm = "0.27" — Cross-platform terminal handling
- rand = "0.8" — Food placement

## License

This project is licensed under the terms of the LICENSE file in this repository.

## Acknowledgements

Thanks to the ratatui and crossterm communities for excellent terminal tooling.
