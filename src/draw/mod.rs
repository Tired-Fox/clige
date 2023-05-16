mod canvas;
pub use canvas::Canvas;

pub fn clear() {
    print!("\x1b[2J")
}

pub fn position(x: u16, y: u16) {
    print!("\x1b[{};{}H", y, x)
}
