mod window;
use window::*;

use std::io;

pub(crate) fn render_loop(height: usize, width: usize) -> io::Result<()> {
    let mut win = Window::new_master(
        height,
        width,
        vec![Window::new_empty(5, 5, 2, 2), Window::new_empty(5, 5, 0, 0)],
    );
    win.children[0].state = State::Active;
    win.draw();
    let mut run = true;
    while run {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        for c in input.trim().chars() {
            let active_pos = win.first_active_child().expect("No active children!");
            let active = &mut win.children[active_pos];

            match c {
                '.' => {
                    if active_pos == 0 {
                        win.change_active(active_pos, win.children.len() - 1);
                    } else {
                        win.change_active(active_pos, active_pos - 1);
                    }
                }
                ',' => {
                    if active_pos == win.children.len() - 1 {
                        win.change_active(active_pos, 0);
                    } else {
                        win.change_active(active_pos, active_pos + 1);
                    }
                }

                'h' => {
                    if active.x != 0 {
                        active.x -= 1
                    } else {
                    }
                }
                'j' => active.y += 1,
                'k' => {
                    if active.y != 0 {
                        active.y -= 1
                    } else {
                    }
                }
                'l' => active.x += 1,

                'H' => {
                    if active.width > 2 {
                        active.width -= 1
                    } else {
                    }
                }
                'J' => active.height += 1,
                'K' => {
                    if active.height > 2 {
                        active.height -= 1
                    } else {
                    }
                }
                'L' => active.width += 1,

                'q' => {
                    if !win.children.is_empty() {
                        win.children.remove(active_pos);
                        if !win.children.is_empty() && active_pos > 0 {
                            win.children[active_pos - 1].state = State::Active;
                        } else if !win.children.is_empty() {
                            win.children[0].state = State::Active;
                        }
                    }
                    if win.children.is_empty() {
                        run = false;
                        println!("\x1b[H\x1b[JGood night.");
                        break;
                    }
                }
                'w' => {
                    win.children.push(Window::new_empty(5, 5, 0, 0));
                    win.change_active(active_pos, win.children.len() - 1);
                }

                _ => {}
            }
            win.draw();
        }
    };
    Ok(())
}
