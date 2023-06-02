use super::{
    color::Color,
    data::{Pixel, Rect},
    get_term_size,
};

// x x x x x
// x x x x x
// x x x x x
// x x x x x
//
// 1. define a size and create a pre allocated buffer of objects
// 2. a way of changing specific indexes
// 3. a way of displaying buffer to stdout/print to screen
//
// 1. init
// 2. update
// 3. render

/// A 2D buffer of data used to store and render data to the terminal
pub trait Buffer {
    type Pixel;
    type Data;
    type Error;

    /// Create a new buffer instance
    ///
    /// The buffer will be fixed at the size specified. The buffer may be resized
    /// but all data will be reset/lost
    fn new(width: usize, height: usize) -> Self;

    /// Set a specific index in the buffer to a new value
    fn set(&mut self, x: usize, y: usize, value: Self::Pixel) -> Result<(), Self::Error>;

    /// Get a specific index in the buffer
    fn get(&mut self, x: usize, y: usize) -> Option<&Self::Pixel>;

    /// Mutably get a specific index in the buffer
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Self::Pixel>;

    /// Render the buffer into a single string
    fn render(&self) -> Result<String, Self::Error>;

    fn width(&self) -> usize;
    fn height(&self) -> usize;

    /// Resize the buffer
    ///
    /// This will **clear the entire buffer** and set a new 2d buffer to the specified
    /// dimensions
    fn resize(&mut self, width: usize, height: usize);
}

/// A pixel buffer.
///
/// Each index in the buffer stores `char` and `Color` data.
#[derive(Clone)]
pub struct PixelBuffer {
    pub pixels: Vec<Pixel>,
    width: usize,
    height: usize,
}

impl Default for PixelBuffer {
    /// Pixel buffer that is the width and height of the terminal
    fn default() -> Self {
        let (width, height) = get_term_size();
        PixelBuffer::new(width, height)
    }
}

impl Buffer for PixelBuffer {
    type Error = String;
    type Data = String;
    type Pixel = Pixel;

    fn resize(&mut self, width: usize, height: usize) {
        self.pixels = vec![Pixel::default(); width * height];
        self.width = width;
        self.height = height;
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn new(width: usize, height: usize) -> Self {
        PixelBuffer {
            pixels: vec![Pixel::default(); width * height],
            width,
            height,
        }
    }

    fn get(&mut self, x: usize, y: usize) -> Option<&Self::Pixel> {
        if x >= self.width || y >= self.height {
            return None;
        }

        self.pixels.get((y * self.width) + x)
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Self::Pixel> {
        if x >= self.width || y >= self.height {
            return None;
        }

        self.pixels.get_mut((y * self.width) + x)
    }

    fn set(&mut self, x: usize, y: usize, value: Self::Pixel) -> Result<(), Self::Error> {
        if x >= self.width || y >= self.height {
            return Err(format!(
                "Index out of bounds; expected x < {} and y < {}",
                self.width, self.height
            ));
        }

        self.pixels[(y * self.width) + x] = value;
        Ok(())
    }

    fn render(&self) -> Result<String, Self::Error> {
        let mut output = String::new();
        let mut color = Color::default();

        for (i, pixel) in self.pixels.iter().enumerate() {
            if pixel.color != color {
                color = pixel.color.clone();
                output.push_str(format!("{}{}", pixel.color, pixel.value).as_str());
            } else {
                output.push(pixel.value);
            }

            if (i + 1) % self.width == 0 && i < self.pixels.len() - 1 {
                output.push('\n');
            }
        }
        Ok(output)
    }
}
