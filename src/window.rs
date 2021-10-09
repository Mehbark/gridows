use std::fmt::write;

#[derive(Debug)]
pub struct Window {
    pub width: usize,
    pub height: usize,
    pub x: usize,
    pub y: usize,
    pub children: Vec<Window>,
    pub state: State,
}

#[derive(Debug)]
pub enum State {
    Inactive,
    Active,
}

impl State {
    /// Returns `true` if the state is [`Inactive`].
    ///
    /// [`Inactive`]: State::Inactive
    pub fn is_inactive(&self) -> bool {
        matches!(self, Self::Inactive)
    }

    /// Returns `true` if the state is [`Active`].
    ///
    /// [`Active`]: State::Active
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }
}

//Creation functions
impl Window {
    fn new_base(
        width: usize,
        height: usize,
        x: usize,
        y: usize,
        children: Option<Vec<Window>>,
    ) -> Self {
        Self {
            width,
            height,
            x,
            y,
            children: children.unwrap_or_else(Vec::new),
            state: State::Inactive,
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

struct StringDisplay {
    vec_form: Vec<Vec<char>>,
    width: usize,
    height: usize,
    background_char: char,
}

impl StringDisplay {
    pub fn new(width: usize, height: usize, background_char: char) -> Self {
        Self {
            vec_form: vec![vec![background_char; width]; height],
            width,
            height,
            background_char,
        }
    }

    pub fn change_char(&mut self, x: usize, y: usize, new_char: char) -> bool {
        if x < self.width && y < self.height {
            self.vec_form[y][x] = new_char;
            return true;
        }
        false
    }

    ///Returns amount successfully changed
    pub fn change_chars_in_slice(
        &mut self,
        from_x: usize,
        to_x: usize,
        y: usize,
        new_char: char,
    ) -> usize {
        if y >= self.height || from_x >= to_x || to_x >= self.width {
            return 0;
        }

        if to_x >= self.width {
            self.vec_form[y].splice(from_x..self.height, vec![new_char; self.width - from_x - 1]);
            return self.width - from_x - 1;
        }

        self.vec_form[y].splice(from_x..to_x, vec![new_char; to_x - from_x]);
        to_x - from_x - 1
    }
}

impl std::fmt::Display for StringDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut rows: Vec<String> = Vec::new();
        for row in &self.vec_form {
            rows.push(row.iter().collect());
        }

        write!(f, "{}", rows.join("\n"))
    }
}

impl Window {
    pub fn render(&self) -> String {
        let mut disp = StringDisplay::new(self.width, self.height, '.');

        for child in &self.children {
            //Corners
            disp.change_char(child.x, child.y, '┌'); //Top-left corner
            disp.change_char(child.x + child.width - 1, child.y, '┐'); //Top-right corner
            disp.change_char(child.x, child.y + child.height - 1, '└'); //Bottom-left corner
            disp.change_char(child.x + child.width - 1, child.y + child.height - 1, '┘'); //Bottom-right corner

            //Sides
            //Top side
            disp.change_chars_in_slice(child.x + 1, child.x + child.width - 1, child.y, '─');
            //Bottom side
            disp.change_chars_in_slice(
                child.x + 1,
                child.x + child.width - 1,
                child.y + child.height - 1,
                '─',
            );
            //Vertical sides
            for row_num in (child.y + 1)..(child.y + child.height - 1) {
                disp.change_char(child.x, row_num, '│'); //Left Side
                disp.change_char(child.x + child.width - 1, row_num, '│'); //Right Side
            }
        }

        format!("{}", &disp)
    }

    pub fn draw(&self) {
        print!(
            "\x1b[H\x1b[J\x1b[{x};{y}H{render}",
            x = self.x,
            y = self.y,
            render = self.render()
        );
    }
}
