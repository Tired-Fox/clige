use super::Viewable;
use std::{cell::RefCell, fmt::Display, rc::Rc};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Text {
    grid: Vec<Vec<Rc<RefCell<char>>>>,
    width: usize,
    height: usize,
    text: String,
    pub x: u16,
    pub y: u16,
}

impl From<String> for Text {
    fn from(value: String) -> Self {
        let mut grid = vec![Vec::new()];
        for char in value.chars() {
            grid[0].push(Rc::new(RefCell::new(char)));
        }

        Text {
            width: grid[0].len(),
            height: 1,
            grid,
            text: value,
            x: 0,
            y: 0,
        }
    }
}

impl From<&str> for Text {
    fn from(value: &str) -> Self {
        value.to_owned().into()
    }
}

impl Text {
    pub fn new() -> Self {
        Text {
            grid: Vec::new(),
            width: 0,
            height: 0,
            text: String::new(),
            x: 0,
            y: 0,
        }
    }

    pub fn builder() -> TextBuilder {
        TextBuilder::new()
    }

    pub fn update(self: &mut Self, text: &str, width: usize) {
        let mut width = width;

        if text.len() <= width {
            width = text.len();
        }


        let mut grid = Vec::new();
        let mut length = grid.len();

        for (i, char) in text.chars().enumerate() {
            if i % width == 0 {
                grid.push(Vec::new());
                length += 1;
            }
            grid[length - 1].push(Rc::new(RefCell::new(char)));
        }

        self.width = width;
        self.height = grid.len();
        self.text = text.to_string();
        self.grid = grid;
    }

    pub fn view(&self) -> &Vec<Vec<Rc<RefCell<char>>>> {
        &self.grid
    }

    pub fn resize(&mut self, x: u16, y: u16, width: usize) {
        let mut width = width;

        if self.text.len() <= width {
            width = self.text.len();
        }

        let mut grid = Vec::new();
        let mut length = grid.len();

        for (i, char) in self.text.chars().enumerate() {
            if i % width == 0 {
                grid.push(Vec::new());
                length += 1;
            }
            grid[length - 1].push(Rc::new(RefCell::new(char)));
        }

        self.x = x;
        self.y= y;
        self.width = width;
        self.height = grid.len();
        self.grid = grid;
    }
}

impl Viewable for Text {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn position(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }
}

pub struct TextBuilder {
    x: u16,
    y: u16,
    text: String,
}

impl TextBuilder {
    pub fn new() -> Self {
        TextBuilder {
            x: 0,
            y: 0,
            text: String::new(),
        }
    }

    pub fn position(mut self: Self, x: u16, y: u16) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn text(mut self: Self, text: &str) -> Self {
        self.text.push_str(text);
        self
    }

    pub fn build(self: Self, width: usize) -> Text {
        let mut width = width;

        if self.text.len() <= width {
            width = self.text.len();
        }

        let mut grid = Vec::new();
        let mut length = grid.len();

        for (i, char) in self.text.chars().enumerate() {
            if i % width == 0 {
                grid.push(Vec::new());
                length += 1;
            }
            grid[length - 1].push(Rc::new(RefCell::new(char)));
        }

        Text {
            height: grid.len(),
            width,
            grid,
            x: self.x,
            y: self.y,
            text: self.text,
        }
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.grid.iter() {
            write!(
                f,
                "{}",
                line.iter().map(|c| *c.borrow()).collect::<String>()
            )?;
        }
        Ok(())
    }
}
