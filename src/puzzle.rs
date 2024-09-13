use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Puzzle {
    grid: Vec<Vec<char>>,
    size: usize,
}

impl Puzzle {
    pub fn new(size: usize) -> Self {
        Puzzle {
            grid: vec![vec![' '; size]; size],
            size,
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: char) {
        self.grid[row][col] = value;
    }

    pub fn get(&self, row: usize, col: usize) -> char {
        self.grid[row][col]
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_solved(&self) -> bool {
        self.grid.iter().all(|row| {
            row.iter()
                .all(|&cell| cell == 'X' || (cell >= 'A' && cell <= 'Z'))
        })
    }
}
