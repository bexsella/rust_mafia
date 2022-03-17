mod rand;

struct GameState {
    
}

impl GameState {
    pub fn new () -> GameState {
        GameState {

        }
    }
}

pub struct Game {
    rng: rand::Rand,

    mob_name: &str,
    player_name: &str,
    player_level: u32,

    pub state: GameState
}

impl Game {
    pub fn tick () {
        
    }
}
