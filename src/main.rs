use window::Window;

mod window;

fn main() {
    let win = Window::new(10, 10, 0, 0, Some(vec![Window::new(1, 1, 1, 1, None)]));
    println!("{:#?}", win);
}
