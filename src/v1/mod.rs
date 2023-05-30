pub mod draw;
pub mod elements;
pub mod ansi;

pub struct Rect {
    pub left: usize,
    pub top: usize,
    pub right: usize,
    pub bottom: usize
}

impl Rect {
    pub fn new(points: Vec<usize>) -> Result<Rect, String> {
        match points.len() {
            // Assume same value for all points
            1 => {
                Ok(Rect {
                    left: points[0],
                    top: points[0],
                    right: points[0],
                    bottom: points[0],
                })
            },
            // Assume both values are width and height
            2 => {
                Ok(Rect {
                    left: 0,
                    top: 0,
                    right: points[0],
                    bottom: points[1],
                })
            },
            4 => {
                Ok(Rect {
                    left: points[0],
                    top: points[1],
                    right: points[2],
                    bottom: points[3],
                })
            },
            _ => {
                Err("Expected vector of length 1, 2, or 4".to_owned())
            }
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
