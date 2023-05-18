extern crate clige;
use clige::{
    ansi::{color, cursor},
    draw::{clear, draw, Home, Style},
    elements::{map::NoiseMap, Canvas, Pixel, Text, Viewable},
};

use ctrlc;
use noise::Perlin;
use std::thread::sleep;
use std::time::Duration;
use std::{
    io::{self, Write},
    sync::atomic::{AtomicBool, Ordering},
};
use std::{process::exit, sync::Arc};

fn main() {
    let noise_scroller = true;
    let element_test = false;

    print!("{}", cursor::HIDE);
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        print!("{}\x1b[0m", cursor::SHOW);
        r.store(false, Ordering::SeqCst);
        exit(3)
    })
    .expect("Error setting ctrl-c handler");

    // !PERF: Noise scroller
    if noise_scroller {
        let mcanv = Canvas::new(true);

        let scale = 100;
        let map = NoiseMap::<Perlin>::perlin(935602)
            .size(mcanv.width() + scale, mcanv.height() + scale)
            .scale(scale as f64 * 0.035)
            .build();

        let view = mcanv.view();

        for i in 0..scale {
            for y in 0..mcanv.height() {
                for x in 0..mcanv.width() {
                    let val = map.get(i + x, i + y);
                    let mut color = (11. * val) as i16;

                    if color < 0 {
                        color = 232 + (color.abs() * 1);
                    } else {
                        color = 232 + 11 + (color * 1);
                    }

                    view[y][x].replace(Pixel::new(
                        Style::foreground(format!("38;5;{}", color)),
                        '█',
                    ));
                }
            }

            // clear();
            Home();
            print!("{}", mcanv);
            io::stdout().flush().unwrap();
            sleep(Duration::from_millis(33));
        }
    }

    // !PERF: Element Test
    if element_test {
        let mut canvas = Canvas::new(true);

        canvas.append(
            Text::builder()
                .position(
                    ((canvas.width() / 2) - 1) as u16,
                    (canvas.height() / 2) as u16,
                )
                .text(vec![Pixel::new(Style::foreground("39"), '0')])
                .build(2)
                .into(),
        );

        let text = canvas.get(0).borrow().to_text().unwrap();

        for color in 1..=255 {
            // Update

            if color % 10 == 0 {
                canvas.toggle_border();
            }

            text.borrow_mut()
                .update(Pixel::colored(Style::foreground(format!("38;5;{}", color)), color.to_string()));

            let width = text.borrow().width();

            text.borrow_mut().move_to(
                ((canvas.width() / 2) - (width / 2)) as u16,
                (canvas.height() / 2) as u16,
            );

            // Render
            draw(&mut canvas);

            // Frame limiter
            sleep(Duration::from_millis(83));
        }

        color::foreground::reset();
    }
    print!("{}\x1b[0m", cursor::SHOW);
}
