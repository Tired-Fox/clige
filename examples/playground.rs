extern crate clige;
use clige::{
    ansi::{color, cursor},
    draw::{clear, position, Canvas},
};

use ctrlc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    clear();
    let canvas = Canvas::new(true);

    let display = canvas.view();
    let mut color: u8 = 0;

    print!("{}", cursor::HIDE);

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        print!("{}\x1b[0m", cursor::SHOW);
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting ctrl-c handler");

    let half = (display.len() / 2) as usize;
    let rlen = display[half].len();

    while running.load(Ordering::SeqCst) {
        color = color + 1;
        if color >= 254 {
            color = 0
        }

        {
            let start = ((rlen - (color.to_string().len() / 2) as usize) / 2) as usize;
            let length = color.to_string().len();

            let clr = color.to_string();
            for i in 0..rlen {
                if i >= start && i < start + length {
                    display[half][i].replace(clr.as_bytes()[i - start] as char);
                } else {
                    display[half][i].replace(' ');
                }
            }
        }

        color::foreground::xterm(1 + color);
        canvas.draw();

        sleep(Duration::from_millis(120));
    }

    clear();
    position(0, 0);
}
