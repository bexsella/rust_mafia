use crate::rand;

use std::io::{self, Write, BufWriter};
use std::fs::File;

pub struct GameState {
    
}

impl GameState {
    pub fn new () -> GameState {
        GameState {

        }
    }
}

pub struct Game {
    state: GameState,

    start_seed: u32,
    rng: rand::Rand,

    mob_name: String,
    player_name: String,
    player_level: u32,
    turns: u128,
}

impl Game {
    pub fn new (seed: u32) ->  Game {
        Game {
            state: GameState::new(),

            start_seed: seed,
            rng: rand::Rand::new(seed),
            mob_name: "The Firm".to_string(),
            player_name: "".to_string(),
            player_level: 1,
            turns: 0
        }
    }

    pub fn tick (&mut self) {
        self.turns += 1;
        self.save_state();
    }

    fn save_state (&self) {
        let file_name = format!("{}{}", self.start_seed, str::replace(&self.player_name, " ", "_"));
        
        if let Ok(mut save_file) = File::create(file_name) {
            write!(&mut save_file, "{}, {}, {}, {}, {}", self.start_seed, self.mob_name, self.player_name, self.player_level, self.turns).expect("Failed to write user save.");
        } else {
            eprintln!("Failed to create save file.");
        }
    }
}
