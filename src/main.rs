mod console;
mod rand;

fn main() {
    let con = console::Console::init();

    // con.set_text_color(console::BACKGROUND_BLUE|console::FOREGROUND_GREEN|console::FOREGROUND_RED|console::FOREGROUND_INTENSITY);
    // con.set_text_position(40, 5);

    con.clear();

    println!("Hello, world!");

    match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(elapsed) => {
            let mut rand = rand::Rand::init(elapsed.as_secs() as u32);

            for _ in 0..1_000_000 {
                print!("{},", rand.next());
            }
        }
        Err(e) => {
            panic!("Time is an aenigma");
        }
    }

    con.quit();
}
