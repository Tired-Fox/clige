use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub enum Context {
    #[default]
    Foreground,
    Background,
    Solid,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub enum SystemColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    #[default]
    White,
}

impl SystemColor {
    pub fn color(&self) -> u8 {
        match self {
            Self::Black => 0,
            Self::Red => 1,
            Self::Green => 2,
            Self::Yellow => 3,
            Self::Blue => 4,
            Self::Magenta => 5,
            Self::Cyan => 6,
            Self::White => 7,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Color {
    foreground: Option<String>,
    background: Option<String>,
}

impl Color {
    pub fn fg(foreground: String) -> Self {
        Color {
            foreground: Some(foreground),
            background: None,
        }
    }

    pub fn bg(background: String) -> Self {
        Color {
            foreground: None,
            background: Some(background),
        }
    }

    pub fn new(foreground: String, background: String) -> Self {
        Color {
            foreground: Some(foreground),
            background: Some(background),
        }
    }

    pub fn xterm(code: u8, context: Context) -> Self {
        match context {
            Context::Foreground => Color::fg(format!("38;5;{}", code)),
            Context::Background => Color::bg(format!("38;5;{}", code)),
            Context::Solid => Color::new(
                format!("38;5;{}", code),
                format!("38;5;{}", code),
            ),
        }
    }

    pub fn rgb(r: u8, g: u8, b: u8, context: Context) -> Self {
        match context {
            Context::Foreground => Color::fg(format!("38;2;{};{};{}", r, g, b)),
            Context::Background => Color::bg(format!("38;2;{};{};{}", r, g, b)),
            Context::Solid => Color::new(
                format!("38;5;{};{};{}", r, g, b),
                format!("38;5;{};{};{}", r, g, b),
            ),
        }
    }

    pub fn system(system: SystemColor, context: Context) -> Self {
        match context {
            Context::Foreground => Color::fg(format!("\x1b[{}m", 30 + system.color())),
            Context::Background => Color::bg(format!("\x1b[{}m", 40 + system.color())),
            Context::Solid => Color::new(
                format!("\x1b[{}m", 30 + system.color()),
                format!("\x1b[{}m", 40 + system.color()),
            ),
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color {
            foreground: Some("\x1b[39m".to_owned()),
            background: Some("\x1b[49m".to_owned()),
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts = Vec::new();
        if let Some(foreground) = &self.foreground {
            parts.push(foreground.clone());
        }

        if let Some(background) = &self.background {
            parts.push(background.clone());
        }

        if parts.len() > 0 {
            write!(f, "\x1b[{}m", parts.join(";"))?;
        }
        Ok(())
    }
}
