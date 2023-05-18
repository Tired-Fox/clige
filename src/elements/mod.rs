mod canvas;
mod text;
pub mod map;

pub use canvas::Canvas;
use std::{
    cell::RefCell,
    fmt::{self, Display},
    rc::Rc,
};
pub use text::Text;

use crate::draw::Style;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum View {
    Text(Rc<RefCell<Text>>),
    Canvas(Rc<RefCell<Canvas>>),
}

impl View {
    pub fn to_text(&self) -> Result<Rc<RefCell<Text>>, String> {
        match self {
            Self::Text(text) => Ok(text.clone()),
            _ => Err("View is not a text object".to_owned()),
        }
    }

    pub fn to_canvas(&self) -> Result<Rc<RefCell<Canvas>>, String> {
        match self {
            Self::Canvas(canvas) => Ok(canvas.clone()),
            _ => Err("View is not a text object".to_owned()),
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

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Pixel {
    pub color: Style,
    pub symbol: char,
}

impl Pixel {
    pub fn new(color: Style, char: char) -> Self
    {
        Pixel {
            color,
            symbol: char,
        }
    }

    pub fn format(&self, prev_color: &Style) -> String {
        let color = self.color.format(prev_color);
        format!("{}{}", color, self.symbol)
    }

    pub fn colored<S>(color: Style, text: S) -> Vec<Pixel>
    where
        S: Display
    {
        text.to_string()
            .chars()
            .map(|c| Pixel {
                color: color.clone(),
                symbol: c,
            })
            .collect()
    }
}

impl From<char> for Pixel {
    fn from(value: char) -> Self {
        Pixel {
            color: Style::default(),
            symbol: value,
        }
    }
}

impl From<&str> for Pixel {
    fn from(value: &str) -> Self {
        if value.len() > 1 {
            Pixel {
                color: Style::new(&value[0..value.len()], ""),
                symbol: value.as_bytes()[value.len() - 1] as char,
            }
        } else {
            Pixel {
                color: Style::default(),
                symbol: value.as_bytes()[0] as char,
            }
        }
    }
}
