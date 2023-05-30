use super::{color::Color, data::{Rect, Pixel}};

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
pub trait Buffer {
    type Pixel;
    type Data;
    type Error;

    fn init(width: usize, height: usize) -> Self;
    fn set(&mut self, x: usize, y: usize, value: Self::Pixel) -> Result<(), Self::Error>;
    fn get(&mut self, x: usize, y: usize) -> Option<&Self::Pixel>;
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Self::Pixel>;
    fn render(&self, view: &Rect) -> Result<String, Self::Error>;

    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

#[derive(Clone)]
pub struct PixelBuffer {
    pub pixels: Vec<Vec<Pixel>>,
    width: usize,
    height: usize,
}

impl Buffer for PixelBuffer {
    type Error = String;
    type Data = String;
    type Pixel = Pixel;

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn init(width: usize, height: usize) -> Self {
        PixelBuffer {
            pixels: vec![vec![Pixel::default(); width]; height],
            width,
            height,
        }
    }

    fn get(&mut self, x: usize, y: usize) -> Option<&Self::Pixel> {
        match self.pixels.get(y) {
            Some(row) => row.get(x),
            None => None,
        }
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Self::Pixel> {
        match self.pixels.get_mut(y) {
            Some(row) => row.get_mut(x),
            None => None,
        }
    }

    fn set(&mut self, x: usize, y: usize, value: Self::Pixel) -> Result<(), Self::Error> {
        if x >= self.width || y >= self.height {
            return Err(format!(
                "Index out of bounds; expected x < {} and y < {}",
                self.width, self.height
            ));
        }

        self.pixels[y][x] = value;
        Ok(())
    }

    fn render(&self, view: &Rect) -> Result<String, Self::Error> {
        let mut output = String::new();
        let mut color = Color::default();

        for line in view.top..=view.bottom {
            if line < self.height() {
                for pixel in view.left..=view.right {
                    if pixel < self.width() {
                        let pixel = &self.pixels[line][pixel];
                        if pixel.color != color {
                            color = pixel.color.clone();
                            output.push_str(format!("{}{}", pixel.color, pixel.value).as_str());
                        } else {
                            output.push(pixel.value);
                        }
                    } else {
                        output.push(' ');
                    }
                }
            } else {
                output.push_str((0..view.height()).map(|_| ' ').collect::<String>().as_str())
            }

            if line != view.bottom {
                output.push_str("\n");
            }
        }

        Ok(output)
    }
}
