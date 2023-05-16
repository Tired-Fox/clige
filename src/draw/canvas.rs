use std::{cell::RefCell, fmt::Display, rc::Rc, io::{self, Write}};

use crate::draw::position;

/// A writable object that stores characters for a terminal grid.
///
/// Allows for displaying a blank canvas or a canvas with a border.
/// The object will only expose the writable portion of the canvase.
/// This means everything but the border character/cells.
#[derive(Debug)]
pub struct Canvas {
    actual: Vec<Vec<Rc<RefCell<char>>>>,
    grid: Vec<Vec<Rc<RefCell<char>>>>,
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
        Canvas::create(0, 0, cols, rows, border).unwrap()
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
    pub fn create(x: u16, y: u16, width: u16, height: u16, border: bool) -> Result<Self, String> {
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

        let mut width = width as usize;
        let mut height = height as usize;

        let mut grid = Vec::new();
        for _ in 0..height {
            grid.push(Vec::new());
            let length = grid.len();

            for _ in 0..width {
                grid[length-1].push(Rc::new(RefCell::new(' ')));
            }

        }

        let mut actual = Vec::new();
        let mut start = 0;

        if border {
            // Assign border characters
            for (i, line) in grid.iter().enumerate() {
                if i == 0 {
                    for (j, char) in line.iter().enumerate() {
                        if j == 0 {
                            char.replace('┌');
                        } else if j == width - 1 {
                            char.replace('┐');
                        } else {
                            char.replace('─');
                        }
                    }
                } else if i == height - 1 {
                    for (j, char) in line.iter().enumerate() {
                        if j == 0 {
                            char.replace('└');
                        } else if j == width - 1 {
                            char.replace('┘');
                        } else {
                            char.replace('─');
                        }
                    }
                } else {
                    let length = line.len();
                    line[0].replace('│');
                    line[length - 1].replace('│');
                }
            }
        
            start = 1;
            height -= 1;
            width -= 1;
        }

        for i in start..height {
            actual.push(Vec::new());
            let length = actual.len();

            for j in start..width {
                actual[length-1].push(grid[i][j].clone())
            }
        }

        Ok(Canvas {
            actual,
            grid,
            width: width.into(),
            height: height.into(),
            x,
            y
        })
    }

    /// Draw the canvas to the standard output
    pub fn draw(self: &Self) {
        position(self.x, self.y);
        print!("{}", self);
        io::stdout().flush().unwrap();
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
    pub fn view(self: &Self) -> Vec<Vec<Rc<RefCell<char>>>> {
        self.actual.clone()
    }
}

impl Display for Canvas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, line) in self.grid.iter().enumerate() {
            write!(
                f,
                "{}",
                line.iter()
                    .map(|c| c.borrow_mut().to_string())
                    .collect::<Vec<String>>()
                    .join("")
            )?;
            if i < self.height {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

impl Default for Canvas {
    fn default() -> Self {
        let termsize::Size { rows, cols } = termsize::get().unwrap();
        Canvas::create(0, 0, cols, rows, false).unwrap()
    }
}
