use rand::{thread_rng, Rng};
use std::collections::VecDeque;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug)]
pub struct Game {
    pub width: i32,
    pub height: i32,
    pub snake: VecDeque<(i32, i32)>,
    pub dir: Direction,
    pub food: (i32, i32),
    pub score: u32,
    pub game_over: bool,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Self {
        let mut snake = VecDeque::new();
        let start_x = width / 2;
        let start_y = height / 2;
        // initial length 3
        snake.push_back((start_x, start_y));
        snake.push_back((start_x - 1, start_y));
        snake.push_back((start_x - 2, start_y));
        let mut g = Self {
            width,
            height,
            snake,
            dir: Direction::Right,
            food: (0, 0),
            score: 0,
            game_over: false,
        };
        g.spawn_food();
        g
    }

    fn spawn_food(&mut self) {
        let mut rng = thread_rng();
        loop {
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.height);
            if !self.snake.iter().any(|&(sx, sy)| sx == x && sy == y) {
                self.food = (x, y);
                break;
            }
        }
    }

    pub fn change_dir(&mut self, new_dir: Direction) {
        // Prevent 180-degree turns
        let opposite = match self.dir {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        };
        if new_dir != opposite {
            self.dir = new_dir;
        }
    }

    pub fn step(&mut self) {
        if self.game_over {
            return;
        }
        let (hx, hy) = self.snake.front().copied().unwrap();
        let (nx, ny) = match self.dir {
            Direction::Up => (hx, hy.saturating_sub(1)),
            Direction::Down => (hx, hy + 1),
            Direction::Left => (hx.saturating_sub(1), hy),
            Direction::Right => (hx + 1, hy),
        };

        // Check walls
        if nx < 0 || ny < 0 || nx >= self.width || ny >= self.height {
            self.game_over = true;
            return;
        }

        // Check self collision
        if self.snake.iter().any(|&(x, y)| x == nx && y == ny) {
            self.game_over = true;
            return;
        }

        // Move
        self.snake.push_front((nx, ny));
        if (nx, ny) == self.food {
            self.score += 1;
            self.spawn_food();
        } else {
            self.snake.pop_back();
        }
    }

    // Render board as text buffer with borders
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        // top border
        out.push('+');
        for _ in 0..self.width {
            out.push('-');
        }
        out.push('+');
        out.push('\n');

        for y in 0..self.height {
            out.push('|');
            for x in 0..self.width {
                if (x, y) == self.food {
                    out.push('*');
                } else if let Some((hx, hy)) = self.snake.front() {
                    if *hx == x && *hy == y {
                        out.push('O');
                        continue;
                    }
                    if self.snake.iter().skip(1).any(|&(sx, sy)| sx == x && sy == y) {
                        out.push('o');
                    } else {
                        out.push(' ');
                    }
                } else {
                    out.push(' ');
                }
            }
            out.push('|');
            out.push('\n');
        }

        // bottom border
        out.push('+');
        for _ in 0..self.width {
            out.push('-');
        }
        out.push('+');
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snake_moves_forward() {
        let mut g = Game::new(10, 10);
        let head_before = g.snake.front().copied().unwrap();
        g.step();
        let head_after = g.snake.front().copied().unwrap();
        assert_ne!(head_before, head_after);
    }

    #[test]
    fn snake_grows_on_food() {
        let mut g = Game::new(10, 10);
        // Place food directly in front of the head
        let (hx, hy) = g.snake.front().copied().unwrap();
        g.dir = Direction::Right;
        g.food = (hx + 1, hy);
        let len_before = g.snake.len();
        g.step();
        assert_eq!(g.snake.len(), len_before + 1);
        assert_eq!(g.score, 1);
    }
}
