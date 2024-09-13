use std::error::Error;
mod game_logic;
mod generator;
mod puzzle;
mod save_load;
mod ui;

use game_logic::Game;
use save_load::{load_game, save_game};

fn main() -> Result<(), Box<dyn Error>> {
    let mut game = load_game().unwrap_or_else(|_| Game::new());
    let mut ui = ui::UI::new()?;

    loop {
        if game.puzzle().is_none() {
            let puzzle = generator::generate_puzzle(game.difficulty);
            game.set_puzzle(puzzle);
        }

        ui.display(&game)?;

        while !game.is_solved() {
            let input = ui.get_input()?;

            if input == 'q' {
                save_game(&game)?;
                return Ok(());
            }

            game.process_input(input);
            ui.display(&game)?;
        }

        ui.display_victory()?;

        if !ui.play_again()? {
            save_game(&game)?;
            break;
        }

        game.increase_difficulty();
    }

    Ok(())
}


