use window::Window;

mod window;

fn main() {
    let win = Window::new_master(
        10,
        10,
        vec![Window::new_empty(5, 5, 2, 2), Window::new_empty(5, 5, 0, 0)],
    );
    println!("{}", win.render());
}
