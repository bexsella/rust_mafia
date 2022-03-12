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

            for _ in 0..1000 {
                print!("{},", rand.get_i32_bounded(0, 100));
            }
        }
        Err(e) => {
            panic!("Time is an aenigma");
        }
    }

    con.quit();
}
