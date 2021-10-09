use window::Window;

mod window;

fn main() {
    let mut win = Window::new_master(
        10,
        10,
        vec![Window::new_empty(5, 5, 2, 2), Window::new_empty(5, 5, 0, 0)],
    );
    win.draw();
    win.children[0].x += 1;
    win.draw();
}
