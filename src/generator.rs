use crate::puzzle::Puzzle;
use rand::Rng;

pub fn generate_puzzle(difficulty: u32) -> Puzzle {
    let size = match difficulty {
        1 => 4,
        2 => 5,
        _ => 6,
    };

    let mut puzzle = Puzzle::new(size);
    let mut rng = rand::thread_rng();

    for _ in 0..(size * size / 3) {
        let row = rng.gen_range(0..size);
        let col = rng.gen_range(0..size);

        if puzzle.get(row, col) == ' ' {
            let value = (b'A' + rng.gen_range(0..26)) as char;

            puzzle.set(row, col, value);
        }
    }

    puzzle
}
