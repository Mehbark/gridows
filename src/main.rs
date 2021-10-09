mod render;
use std::io;

fn main() -> io::Result<()> {
    render::render_loop(30, 30)
}
