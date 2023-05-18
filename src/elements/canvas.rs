use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::draw::Style;

use super::{Pixel, View, Viewable};

const FILL: char = ' ';

/// A writable object that stores characters for a terminal grid.
///
/// Allows for displaying a blank canvas or a canvas with a border.
/// The object will only expose the writable portion of the canvase.
/// This means everything but the border character/cells.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Canvas {
    actual: Vec<Vec<Rc<RefCell<Pixel>>>>,
    grid: Vec<Vec<Rc<RefCell<Pixel>>>>,
    pub border: bool,
    border_style: Style,
    pub children: Vec<Rc<RefCell<View>>>,
    pub width: usize,
    pub height: usize,
    pub x: u16,
    pub y: u16,
}

impl Canvas {
    /// Create a new canvas the size of the terminal.
    ///
    /// If the border is set to true the viewable/writable portion of the canvas
    /// will have a width two less then the terminal and a height of 2 less
    /// than the terminal.
    pub fn new(border: bool) -> Self {
        let termsize::Size { rows, cols } = termsize::get().unwrap();
        Canvas::create(0, 0, cols, rows, border, Style::default()).unwrap()
    }

    /// Create a new Canvas
    ///
    /// The position of the canvas is only used for nested canvases.
    /// The width and height of the canvas must be less than the terminal.
    /// If the border is set to true the viewable/writable portion of the canvas
    /// will have a width two less then what is specified and a height of 2 less
    /// than what is specified.
    ///
    /// # Args
    /// - x: The x position of the nested canvas
    /// - y: The y position of the nested canvas
    /// - width: The width of the canvas
    /// - height: The height of the canvas
    /// - border: Whether the border should be drawn
    pub fn create(
        x: u16,
        y: u16,
        width: u16,
        height: u16,
        border: bool,
        border_style: Style,
    ) -> Result<Self, String> {
        let termsize::Size { rows, cols } = termsize::get().unwrap();
        if width > cols {
            return Err(format!(
                "Width must be less then the terminals current width: {}",
                cols
            ));
        }

        if height > rows {
            return Err(format!(
                "Height must be less than the terminals current height: {}",
                rows
            ));
        }

        let width = width as usize;
        let height = height as usize;

        let mut grid = Vec::new();
        for _ in 0..height {
            grid.push(Vec::new());
            let length = grid.len();

            for _ in 0..width {
                grid[length - 1].push(Rc::new(RefCell::new(FILL.into())));
            }
        }

        let mut actual = Vec::new();

        if border {
            for i in 1..height - 1 {
                actual.push(Vec::new());
                let length = actual.len();

                for j in 1..width - 1 {
                    actual[length - 1].push(grid[i][j].clone())
                }
            }
        } else {
            for i in 0..height {
                actual.push(Vec::new());
                let length = actual.len();

                for j in 0..width {
                    actual[length - 1].push(grid[i][j].clone())
                }
            }
        }

        let mut canvas = Canvas {
            actual,
            grid,
            children: Vec::new(),
            width: width.into(),
            height: height.into(),
            border,
            border_style,
            x,
            y,
        };

        canvas.border();
        Ok(canvas)
    }

    pub fn reset(&mut self) {
        for line in self.actual.iter() {
            for char in line {
                char.replace(FILL.into());
            }
        }
    }

    pub fn update_border_style(&mut self, style: Style) {
        self.border_style = style;
        self.border()
    }

    pub fn toggle_border(&mut self) {
        self.border = !self.border;
        self.border();

        // Adjust actual viewport
        let mut actual = Vec::new();
        if self.border {
            for i in 1..self.height - 1 {
                actual.push(Vec::new());
                let length = actual.len();

                for j in 1..self.width - 1 {
                    actual[length - 1].push(self.grid[i][j].clone())
                }
            }
        } else {
            for i in 0..self.height {
                actual.push(Vec::new());
                let length = actual.len();

                for j in 0..self.width {
                    actual[length - 1].push(self.grid[i][j].clone())
                }
            }
        }
        self.actual = actual
    }

    fn border(&mut self) {
        if self.border {
            // Assign border characters
            for (i, line) in self.grid.iter().enumerate() {
                if i == 0 {
                    for (j, char) in line.iter().enumerate() {
                        if j == 0 {
                            char.replace(Pixel::new(self.border_style.clone(), '┌'));
                        } else if j == self.width - 1 {
                            char.replace(Pixel::new(self.border_style.clone(), '┐'));
                        } else {
                            char.replace(Pixel::new(self.border_style.clone(), '─'));
                        }
                    }
                } else if i == self.height - 1 {
                    for (j, char) in line.iter().enumerate() {
                        if j == 0 {
                            char.replace(Pixel::new(self.border_style.clone(), '└'));
                        } else if j == self.width - 1 {
                            char.replace(Pixel::new(self.border_style.clone(), '┘'));
                        } else {
                            char.replace(Pixel::new(self.border_style.clone(), '─'));
                        }
                    }
                } else {
                    let length = line.len();
                    line[0].replace(Pixel::new(self.border_style.clone(), '│'));
                    line[length - 1].replace(Pixel::new(self.border_style.clone(), '│'));
                }
            }
        } else {
            for (i, line) in self.grid.iter().enumerate() {
                if i == 0 {
                    for char in line.iter() {
                        char.replace(FILL.into());
                    }
                } else if i == self.height - 1 {
                    for char in line.iter() {
                        char.replace(FILL.into());
                    }
                } else {
                    let length = line.len();
                    line[0].replace(FILL.into());
                    line[length - 1].replace(FILL.into());
                }
            }
        }
    }

    pub fn append(self: &mut Self, child: View) {
        self.children.push(Rc::new(RefCell::new(child)))
    }

    pub fn remove(self: &mut Self, child: View) -> View {
        let index = self
            .children
            .iter()
            .position(|v| *v.borrow() == child)
            .unwrap();
        self.children.remove(index).borrow().clone()
    }

    pub fn get(self: &Self, index: usize) -> Rc<RefCell<View>> {
        self.children[index].clone()
    }

    /// The writable portion of the canvas.
    ///
    /// The returned 2D vector has references to the characters on the canvas
    /// that can be edited. It is recommended to call `char.replace('')` when
    /// changing the value of any given index. The changes are directly reflected
    /// on the canvas.
    ///
    /// The only time this 2D vector will be smaller then the specified canvas Size
    /// is if there is a border or other canvas decoration
    ///
    /// # Example
    /// ```
    /// // assume a size of 1 by 5
    /// let canvas = Canvas::new();
    /// let view = canvas.view();
    ///
    /// // Canvas
    /// //     |
    ///
    /// view[0][0].replace('%');
    ///
    /// // Canvas
    /// //%    |
    /// ```
    pub fn view(&self) -> &Vec<Vec<Rc<RefCell<Pixel>>>> {
        &self.actual
    }
}

impl Display for Canvas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut color = Style::default();
        for (i, line) in self.grid.iter().enumerate() {
            write!(
                f,
                "{}",
                line.iter()
                    .map(|c| {
                        let char = (*c.borrow()).format(&color);
                        color = c.borrow().color.clone();
                        char
                    })
                    .collect::<Vec<String>>()
                    .join("")
            )?;

            if i < self.height - 1 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

impl Default for Canvas {
    fn default() -> Self {
        let termsize::Size { rows, cols } = termsize::get().unwrap();
        Canvas::create(0, 0, cols, rows, false, Style::default()).unwrap()
    }
}

impl Viewable for Canvas {
    fn width(&self) -> usize {
        match self.border {
            true => self.width - 2,
            false => self.width
        }
    }

    fn height(&self) -> usize {
        match self.border {
            true => self.height - 2,
            false => self.height
        }
    }

    fn position(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }
}
