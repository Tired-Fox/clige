extern crate clige;

use std::io::Write;
use std::time::{Duration, Instant};
use std::{io, thread::sleep};

use clige::core::{
    buffer::{Buffer, PixelBuffer},
    color::{Color, Context},
    data::{Pixel, Rect},
    noise_map::NoiseMap,
};
use rand::{thread_rng, Rng};

fn update(dt: f32) -> Result<(), String> {
    Ok(())
}

fn main() {
    let termsize::Size { rows, cols } = termsize::get().unwrap();
    let mut buffer = PixelBuffer::init(cols as usize, rows as usize);

    let scale = 100;
    let noise = NoiseMap::perlin(thread_rng().gen::<u32>())
        .size(1000, 1000)
        .scale(scale as f64 * 0.035)
        .build();

    print!("\x1b[?25h");

    let frame_rate: f32 = 1. / 12.; // 12 frames per second
    for i in 0..scale {
        let start = Instant::now();
        print!("\x1b[H");
        for h in 0..buffer.height() {
            for w in 0..buffer.width() {
                let mut n = (11. * noise.get(i + w, i + h)) as i16;

                if n < 0 {
                    n = 232 + (n.abs() * 1);
                } else {
                    n = 232 + 11 + (n * 1);
                }

                buffer
                    .set(
                        w,
                        h,
                        Pixel {
                            value: '█',
                            color: Color::xterm(n as u8, Context::Foreground),
                        },
                    )
                    .unwrap();
            }
        }
        print!(
            "{}",
            buffer
                .render(&Rect::from([buffer.width(), buffer.height()]))
                .unwrap()
        );
        io::stdout().flush().unwrap();

        let dt = (1000. * frame_rate) - start.elapsed().as_millis() as f32;
        if dt > 0. {
            sleep(Duration::from_millis(dt as u64));
            update(frame_rate).unwrap();
        } else if dt > 0. {
            update(dt).unwrap();
            panic!("Took too long");
        }
    }

    print!("\x1b[0m");
    // print!("\x1b[2J\x1b[H");
    print!("\x1b[?25l");
    println!("({}, {})", buffer.width(), buffer.height());
}
