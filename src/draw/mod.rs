mod canvas;
mod text;
mod render;
pub use canvas::Canvas;
pub use text::Text;
pub use render::draw;

use std::{rc::Rc, cell::RefCell, fmt::{self, Display}};

pub fn clear() {
    print!("\x1b[2J")
}

pub fn position(x: u16, y: u16) {
    print!("\x1b[{};{}H", y, x)
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum View {
    Text(Rc<RefCell<Text>>),
    Canvas(Rc<RefCell<Canvas>>)
}

impl View {
    pub fn to_text(&self) -> Result<Rc<RefCell<Text>>, String> {
        match self {
            Self::Text(text) => Ok(text.clone()),
            _ => Err("View is not a text object".to_owned())
        }

    }

    pub fn to_canvas(&self) -> Result<Rc<RefCell<Canvas>>, String> {
        match self {
            Self::Canvas(canvas) => Ok(canvas.clone()),
            _ => Err("View is not a text object".to_owned())
        }

    }
}

impl From<Text> for View {
    fn from(value: Text) -> Self {
        View::Text(Rc::new(RefCell::new(value)))
    }
}

impl From<Canvas> for View {
    fn from(value: Canvas) -> Self {
        View::Canvas(Rc::new(RefCell::new(value)))
    }
}

pub trait Viewable: fmt::Debug + Display {
    fn height(&self) -> usize;
    fn width(&self) -> usize;
    fn position(&self) -> (usize, usize);
}
