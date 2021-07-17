use crate::game;
use std::{fs, io, path};

pub struct NotFound;

pub fn load() -> Result<game::Game, NotFound> {
    let data: Vec<u8> = read()?;
    let game = serde_json::from_slice(&data).unwrap();
    Ok(game)
}

pub fn save(game: &game::Game) -> Result<(), io::Error> {
    let data = serde_json::to_vec(game).unwrap();
    write(data)
}

pub fn remove() {
    let rpg_dir = rpg_dir();
    if rpg_dir.exists() {
        fs::remove_dir_all(&rpg_dir).unwrap();
    }
}

fn read() -> Result<Vec<u8>, NotFound> {
    fs::read(file()).map_err(|_| NotFound)
}

fn write(data: Vec<u8>) -> Result<(), io::Error> {
    let rpg_dir = rpg_dir();
    if !rpg_dir.exists() {
        fs::create_dir(&rpg_dir).unwrap();
    }
    fs::write(file(), &data)
}

fn rpg_dir() -> path::PathBuf {
    dirs::home_dir().unwrap().join(".rpg")
}

fn file() -> path::PathBuf {
    rpg_dir().join("data")
}