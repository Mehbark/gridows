mod window;
use std::io::prelude::*;
use std::io::{self, Read};
use window::Window;

use crate::window::State;

fn main() -> io::Result<()> {
    let mut win = Window::new_master(
        10,
        10,
        vec![Window::new_empty(5, 5, 2, 2), Window::new_empty(5, 5, 0, 0)],
    );

    let mut buffer;
    let mut inp;
    win.children[0].state = State::Active;

    loop {
        buffer = vec![0];
        inp = 0;

        while inp == 0 {
            inp = io::stdin().read(&mut buffer)?;
        }

        win.draw();
        println!(
            "{}",
            String::from_utf8(buffer)
                .expect("failed to parse input")
                .trim()
        );
    }
}
