use crate::draw::Style;

use super::{Pixel, Viewable};
use std::{cell::RefCell, fmt::Display, rc::Rc};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Text {
    grid: Vec<Vec<Rc<RefCell<Pixel>>>>,
    width: usize,
    text: String,
    pixels: Vec<Pixel>,
    pub x: u16,
    pub y: u16,
}

impl From<String> for Text {
    fn from(value: String) -> Self {
        let mut grid = vec![Vec::new()];
        let pixels = Pixel::colored(Style::default(), value.clone());
        for char in pixels.iter() {
            grid[0].push(Rc::new(RefCell::new(char.clone())));
        }

        Text {
            width: grid[0].len(),
            grid,
            text: value,
            pixels,
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
            text: String::new(),
            pixels: Vec::new(),
            x: 0,
            y: 0,
        }
    }

    pub fn builder() -> TextBuilder {
        TextBuilder::new()
    }

    pub fn update(self: &mut Self, pixels: Vec<Pixel>) {
        self.text = pixels.iter().map(|c| c.symbol).collect::<String>();
        self.pixels = pixels;

        self.redraw();
    }

    pub fn view(&self) -> &Vec<Vec<Rc<RefCell<Pixel>>>> {
        &self.grid
    }

    pub fn move_to(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }

    fn redraw(&mut self) {
        let mut width = self.width;
        if self.pixels.len() < width {
            width = self.pixels.len()
        }

        let mut grid = Vec::new();
        let mut length = grid.len();

        for (i, char) in self.pixels.iter().enumerate() {
            if i % width == 0 {
                grid.push(Vec::new());
                length += 1;
            }
            grid[length - 1].push(Rc::new(RefCell::new(char.clone())));
        }

        self.grid = grid
    }

    pub fn resize(&mut self, width: usize) -> usize {
        self.width = width;
        self.redraw();
        self.width
    }
}

impl Viewable for Text {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    fn position(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }
}

pub struct TextBuilder {
    x: u16,
    y: u16,
    pixels: Vec<Pixel>
}

impl TextBuilder {
    pub fn new() -> Self {
        TextBuilder {
            x: 0,
            y: 0,
            pixels: Vec::new(),
        }
    }

    pub fn position(mut self, x: u16, y: u16) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn text(mut self, pixels: Vec<Pixel>) -> Self {
        self.pixels = pixels;
        self
    }

    pub fn build(self, width: usize) -> Text {
        let mut fixed_width = width;

        if self.pixels.len() <= fixed_width {
            fixed_width = self.pixels.len();
        }

        let mut grid = Vec::new();
        let mut length = grid.len();

        for (i, char) in self.pixels.iter().enumerate() {
            if i % fixed_width == 0 {
                grid.push(Vec::new());
                length += 1;
            }
            grid[length - 1].push(Rc::new(RefCell::new(char.to_owned())));
        }

        Text {
            width,
            grid,
            x: self.x,
            y: self.y,
            text: self.pixels.iter().map(|p| p.symbol).collect::<String>(),
            pixels: self.pixels,
        }
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.grid.iter() {
            write!(
                f,
                "{}",
                line.iter()
                    .map(|c| (*c.borrow()).symbol)
                    .collect::<String>()
            )?;
        }
        Ok(())
    }
}
