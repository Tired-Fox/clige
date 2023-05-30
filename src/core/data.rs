use super::color::Color;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Pixel {
    pub value: char,
    pub color: Color,
}

impl Pixel {
    pub fn new(value: char, color: Color) -> Self {
        Pixel { color, value }
    }
}

impl From<char> for Pixel {
    fn from(value: char) -> Self {
        Pixel::new(value, Color::default())
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Pixel {
            color: Color::default(),
            value: ' ',
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Rect {
    pub left: usize,
    pub top: usize,
    pub right: usize,
    pub bottom: usize,
}

impl Rect {
    pub fn new() -> Rect {
        Rect {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0
        }
    }

    pub fn clamp(&mut self, width: usize, height: usize) {
        self.left = self.left.min(width).max(0);
        self.right = self.right.min(width).max(0);
        self.top = self.top.min(height).max(0);
        self.bottom = self.bottom.min(height).max(0);
    }

    pub fn width(&self) -> usize {
        self.right - self.left
    }

    pub fn height(&self) -> usize {
        self.bottom - self.top
    }
}

impl From<[usize; 1]> for Rect {
    fn from(value: [usize; 1]) -> Self {
        Rect {
            left: value[0],
            top: value[0],
            right: value[0],
            bottom: value[0],
        }
    }
}

impl From<[usize; 2]> for Rect {
    fn from(value: [usize; 2]) -> Self {
        Rect {
            left: 0,
            top: 0,
            right: value[0] - 1,
            bottom: value[1] - 1,
        }
    }
}

impl From<[usize; 4]> for Rect {
    fn from(value: [usize; 4]) -> Self {
        Rect {
            left: value[0],
            top: value[1],
            right: value[2],
            bottom: value[3],
        }
    }
}
