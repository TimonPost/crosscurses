extern crate crosscurses;

use crosscurses::{initscr, endwin};

fn main() {
    let window = initscr();
    window.printw("Hello Rust");
    window.refresh();
    window.getch();
    endwin();
}
