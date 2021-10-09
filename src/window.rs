#[derive(Debug)]
pub struct Window {
    pub width: usize,
    pub height: usize,
    pub x: usize,
    pub y: usize,
    pub children: Vec<Window>,
}

//Creation functions
impl Window {
    fn new_base(
        width: usize,
        height: usize,
        x: usize,
        y: usize,
        children: Option<Vec<Window>>,
    ) -> Window {
        Window {
            width,
            height,
            x,
            y,
            children: children.unwrap_or_else(Vec::new),
        }
    }

    pub fn new(width: usize, height: usize, x: usize, y: usize, children: Vec<Window>) -> Window {
        Window::new_base(width, height, x, y, Some(children))
    }
    pub fn new_empty(width: usize, height: usize, x: usize, y: usize) -> Window {
        Window::new_base(width, height, x, y, None)
    }

    pub fn new_master(width: usize, height: usize, children: Vec<Window>) -> Window {
        Window::new_base(width, height, 0, 0, Some(children))
    }

    pub fn new_master_empty(width: usize, height: usize) -> Window {
        Window::new_base(width, height, 0, 0, None)
    }
}

impl Window {
    pub fn render(&self) -> String {
        let mut disp = vec![vec!["."; self.width]; self.height];
        for child in &self.children {
            //Corners
            disp[child.y][child.x] = "┌"; //Top-left corner
            disp[child.y][child.x + child.width - 1] = "┐"; //Top-right corner
            disp[child.y + child.height - 1][child.x] = "└"; //Bottom-left corner
            disp[child.y + child.height - 1][child.x + child.width - 1] = "┘"; //Bottom-right corner

            //Sides
            //Top side
            disp[child.y].splice(
                (child.x + 1)..(child.x + child.width - 1),
                vec!["─"; child.width - 2],
            );
            //Bottom side
            disp[child.y + child.height - 1].splice(
                (child.x + 1)..(child.x + child.width - 1),
                vec!["─"; child.width - 2],
            );
            //Vertical sides
            for row in &mut disp[(child.y + 1)..(child.y + child.height - 1)] {
                row[child.x] = "│"; //Left Side
                row[child.x + child.width - 1] = "│"; //Right Side
            }
        }

        let mut rows = Vec::new();
        for row in &disp {
            rows.push(row.join(""));
        }

        rows.join("\n")
    }

    pub fn draw(&self) {
        print!("\x1b[J\x1b[{x};{y}H{render}", x = self.x, y = self.y, render = self.render());
    }
}
