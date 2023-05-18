pub mod cursor {
    pub const HIDE: &str = "\x1b[?25h";
    pub const SHOW: &str = "\x1b[?25l";
}

pub mod color {
    pub mod foreground {
        pub fn xterm(code: u8) {
            print!("\x1b[38;5;{}m", code)
        }

        pub fn rgb(r: u8, g: u8, b: u8) {
            print!("\x1b[38;2;{};{};{}m", r, g, b)
        }

        pub fn reset() {
            print!("\x1b[22;24;39m")
        }

        const BLACK: &str = "\x1b[30m";
        const RED: &str = "\x1b[31m";
        const GREEN: &str = "\x1b[32m";
        const YELLOW: &str = "\x1b[33m";
        const BLUE: &str = "\x1b[34m";
        const MAGENTA: &str = "\x1b[35m";
        const CYAN: &str = "\x1b[36m";
        const WHITE: &str = "\x1b[37m";
    }

    pub mod background {
        pub fn xterm(code: u8) {
            print!("\x1b[48;5;{}m", code)
        }

        pub fn rgb(r: u8, g: u8, b: u8) {
            print!("\x1b[48;2;{};{};{}m", r, g, b)
        }

        const BLACK: &str = "\x1b[40m";
        const RED: &str = "\x1b[41m";
        const GREEN: &str = "\x1b[42m";
        const YELLOW: &str = "\x1b[43m";
        const BLUE: &str = "\x1b[44m";
        const MAGENTA: &str = "\x1b[45m";
        const CYAN: &str = "\x1b[46m";
        const WHITE: &str = "\x1b[47m";
    }
}

pub fn point(x: u32, y: u32) -> String {
    format!("\x1b[{};{}H", y, x)
}
