mod render;
use std::fmt::Display;

pub use render::{draw, render};

use crate::ansi::color::background;

pub fn clear() {
    print!("\x1b[H\x1b[0m\x1b[2J");
}

pub fn Home() {
    print!("\x1b[H")
}

pub fn position(x: u16, y: u16) {
    print!("\x1b[{};{}H", y, x)
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Style {
    pub foreground: String,
    pub background: String,
}

impl Style {
    pub fn format(&self, previous: &Self) -> String {
        let mut format = Vec::new();
        if self.foreground != previous.foreground && self.foreground.trim() != "" {
            format.push(self.foreground.clone());
        }

        if self.background != previous.background && self.background.trim() != "" {
            format.push(self.background.clone());
        }

        if format.len() > 0 {
            return format!("\x1b[{}m", format.join(";"));
        }
        String::new()
    }

    pub fn foreground<S>(color: S) -> Self
    where
        S: Display,
    {
        Style {
            foreground: color.to_string(),
            background: String::new()
        }
    }

    pub fn background<S>(color: S) -> Self
    where
        S: Display,
    {
        Style {
            foreground: color.to_string(),
            background: String::new()
        }
    }

    pub fn new<S, T>(fg: S, bg: T) -> Self
    where
        S: Display,
        T: Display,
    {
        Style {
            foreground: fg.to_string(),
            background: bg.to_string(),
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Style {
            foreground: "39".to_owned(),
            background: "49".to_owned(),
        }
    }
}
