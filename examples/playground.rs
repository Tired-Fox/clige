extern crate clige;
use clige::{
    ansi::{color, cursor},
    draw::{clear, position, Canvas, Text, Viewable, draw},
};

use ctrlc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    clear();
    let mut canvas = Canvas::new(true);

    canvas.append(
        Text::builder()
            .position(
                ((canvas.width() / 2) - 1) as u16,
                (canvas.height() / 2) as u16,
            )
            .text("0")
            .build(3)
            .into(),
    );

    let text = canvas.get(0).borrow().to_text().unwrap();

    print!("{}", cursor::HIDE);

    let mut color: u8 = 0;
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        print!("{}\x1b[0m", cursor::SHOW);
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting ctrl-c handler");

    while running.load(Ordering::SeqCst) {
        color += 1;
        if color >= 254 {
            color = 0
        }

        if color % 10 == 0 {
            canvas.toggle_border()
        }

        text.borrow_mut().update(color.to_string().as_str(), 2);
        let width = text.borrow().width();
        text.borrow_mut().resize(
            ((canvas.width() / 2) - (width/2)) as u16,
            (canvas.height() / 2) as u16,
            width);

        color::foreground::xterm(1 + color);
        draw(&mut canvas);

        sleep(Duration::from_millis(120));
    }

    clear();
    position(0, 0);
}
