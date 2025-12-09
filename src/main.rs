mod game;

use crate::game::{Direction, Game};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, terminal};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Alignment, Constraint, Direction as LayoutDirection, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Terminal;
use std::io::{stdout, Stdout};
use std::time::{Duration, Instant};

fn setup_terminal() -> crossterm::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(mut terminal: Terminal<CrosstermBackend<Stdout>>) -> crossterm::Result<()> {
    terminal.show_cursor()?;
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}

fn main() -> crossterm::Result<()> {
    let mut terminal = setup_terminal()?;

    let size = terminal.size()?;
    // Leave room for borders in our ASCII renderer
    let grid_w = (size.width.saturating_sub(4)).min(60) as i32;
    let grid_h = (size.height.saturating_sub(6)).min(30) as i32;

    let mut game = Game::new(grid_w.max(10), grid_h.max(10));

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(120);

    loop {
        terminal.draw(|f| {
            let area = f.size();
            let chunks = Layout::default()
                .direction(LayoutDirection::Vertical)
                .constraints(
                    [
                        Constraint::Length(1),
                        Constraint::Min(3),
                        Constraint::Length(1),
                    ]
                    .as_ref(),
                )
                .split(area);

            let header = Paragraph::new(Line::from(vec![
                Span::styled("Rust TUI Snake — ", Style::default().fg(Color::Cyan)),
                Span::raw(format!("Score: {}", game.score)),
            ]))
            .alignment(Alignment::Center);
            f.render_widget(header, chunks[0]);

            // Render the game board as text
            let board_text = game.render_text();
            let board = Paragraph::new(board_text)
                .block(Block::default().borders(Borders::ALL).title("Board"));
            f.render_widget(board, chunks[1]);

            let footer = if game.game_over {
                Paragraph::new("Game Over — press R to restart or Q to quit")
                    .alignment(Alignment::Center)
                    .style(Style::default().fg(Color::Red))
            } else {
                Paragraph::new("Use Arrow Keys / WASD to move • Q to quit")
                    .alignment(Alignment::Center)
            };
            f.render_widget(footer, chunks[2]);
        })?;

        // Input handling
        while event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            restore_terminal(terminal)?;
                            return Ok(());
                        }
                        KeyCode::Char('r') if game.game_over => {
                            // restart
                            let size = terminal.size()?;
                            let grid_w = (size.width.saturating_sub(4)).min(60) as i32;
                            let grid_h = (size.height.saturating_sub(6)).min(30) as i32;
                            game = Game::new(grid_w.max(10), grid_h.max(10));
                        }
                        KeyCode::Up | KeyCode::Char('w') => game.change_dir(Direction::Up),
                        KeyCode::Down | KeyCode::Char('s') => game.change_dir(Direction::Down),
                        KeyCode::Left | KeyCode::Char('a') => game.change_dir(Direction::Left),
                        KeyCode::Right | KeyCode::Char('d') => game.change_dir(Direction::Right),
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            if !game.game_over {
                game.step();
            }
            last_tick = Instant::now();
        }
    }
}
