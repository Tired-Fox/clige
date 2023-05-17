use super::Canvas;
use std::io::{self, Write};

pub fn draw(canvas: &mut Canvas) {
    canvas.render();
    print!("{}", canvas);
    io::stdout().flush().unwrap();
}
