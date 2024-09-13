use crate::puzzle::Puzzle;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Game {
    puzzle: Option<Puzzle>,
    pub difficulty: u32,
    cursor: (usize, usize),
}

impl Game {
    pub fn new() -> Self {
        Game {
            puzzle: None,
            difficulty: 1,
            cursor: (0, 0),
        }
    }

    pub fn set_puzzle(&mut self, puzzle: Puzzle) {
        self.puzzle = Some(puzzle);
        self.cursor = (0, 0);
    }

    pub fn puzzle(&self) -> Option<&Puzzle> {
        self.puzzle.as_ref()
    }

    pub fn cursor(&self) -> (usize, usize) {
        self.cursor
    }

    fn move_cursor(&mut self, (dy, dx): (i32, i32), size: usize) {
        let (y, x) = self.cursor;

        self.cursor = (
            (y as i32 + dy).rem_euclid(size as i32) as usize,
            (x as i32 + dx).rem_euclid(size as i32) as usize,
        )
    }

    pub fn toggle_cell(&mut self) {
        if let Some(puzzle) = &mut self.puzzle {
            let (y, x) = self.cursor;
            let current = puzzle.get(y, x);

            if current == ' ' {
                puzzle.set(y, x, 'X');
            } else if current == 'X' {
                puzzle.set(y, x, ' ');
            }
        }
    }

    pub fn process_input(&mut self, input: char) {
        if let Some(puzzle) = &self.puzzle {
            let size = puzzle.size();

            match input {
                'w' => self.move_cursor((-1, 0), size),
                's' => self.move_cursor((1, 0), size),
                'a' => self.move_cursor((0, -1), size),
                'd' => self.move_cursor((0, 1), size),
                ' ' => self.toggle_cell(),
                _ => {}
            }
        }
    }

    pub fn is_solved(&self) -> bool {
        self.puzzle.as_ref().map_or(false, |p| p.is_solved())
    }

    pub fn increase_difficulty(&mut self) {
        self.difficulty = (self.difficulty % 3) + 1;
        self.puzzle = None;
    }
}
