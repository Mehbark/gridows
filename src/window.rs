#[derive(Debug)]
pub struct Window {
    width: u32,
    height: u32,
    x: u32,
    y: u32,
    children: Vec<Window>,
}

impl Window {
    pub fn new(width: u32, height: u32, x: u32, y: u32, children: Option<Vec<Window>>) -> Window {
        Window {
            width,
            height,
            x,
            y,
            children: children.unwrap_or_else(Vec::new),
        }
    }

    pub fn new_empty(width: u32, height: u32, x: u32, y: u32) -> Window {
        Window::new(width, height, x, y, None)
    }

    pub fn new_master(width: u32, height: u32, children: Option<Vec<Window>>) -> Window {
        Window::new(width, height, 0, 0, children)
    }

    pub fn new_master_empty(width: u32, height: u32) -> Window {
        Window::new(width, height, 0, 0, None)
    }

}
