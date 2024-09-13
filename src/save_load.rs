use crate::game_logic::Game;
use std::{fs, io};

pub fn save_game(game: &Game) -> io::Result<()> {
    let serialized = serde_json::to_string(game)?;
    fs::write("game_save.json", serialized)?;

    Ok(())
}

pub fn load_game() -> io::Result<Game> {
    let serialized = fs::read_to_string("game_save.json")?;
    let game: Game = serde_json::from_str(&serialized)?;

    Ok(game)
}
