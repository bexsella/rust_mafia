mod console;
mod rand;
use std::time;


fn main() {
    let con = console::Console::new();

    let now = time::SystemTime::now();
    let seed = now.elapsed().unwrap_or(time::Duration::from_secs(65536));

    // con.set_text_color(console::BACKGROUND_BLUE|console::FOREGROUND_GREEN|console::FOREGROUND_RED|console::FOREGROUND_INTENSITY);
    con.set_text_position(40, 5);

    let mut quit = false;

    while !quit {
        con.clear();

        let key = con.read_key();

        if key.0 == true {
            if key.1 == 27 {
                quit = true;
            }
        }

        print!("Time has passed.\n")
    }

    con.quit();
}
