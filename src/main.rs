mod console;

fn main() {
    let con = console::Console::init();

    con.set_text_color(console::BACKGROUND_BLUE|console::FOREGROUND_GREEN|console::FOREGROUND_RED|console::FOREGROUND_INTENSITY);
    con.set_text_position(40, 5);

    println!("Hello, world!");

    con.quit();
}
