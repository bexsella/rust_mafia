mod console;
mod rand;
mod mafia;

use std::time;

fn main() {
    let con = console::Console::new();

    let now = time::SystemTime::now();
    let seed = match now.duration_since(time::UNIX_EPOCH) {
        Ok(time) => time,
        Err(_) => time::Duration::from_secs(65536),
    }.as_secs();
    
    con.clear();

    let mut g = mafia::Game::new(seed as u32);

    let mut quit = false;

    while !quit {
        con.set_text_position(0, 0);
        println!("Active seed is: {}", seed);

        let key = con.read_key();

        if key.0 == true {
            if key.1 == 27 {
                quit = true;
            } else {
                g.tick()
            }
        }

        con.clear();
    }

    con.quit();
}
