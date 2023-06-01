extern crate clige;

use std::io::Write;
use std::time::{Duration, Instant};
use std::{io, thread::sleep};

use clige::core::{
    buffer::{Buffer, PixelBuffer},
    color::{Color, Context},
    data::Pixel,
    noise_map::NoiseMap,
};
use rand::{thread_rng, Rng};

// fn update(dt: f32) -> Result<(), String> {
//     Ok(())
// }

fn main() {
    // ┄ ┆
    // ═ ║

 //    print!("\x1b[33;40m");
 //    println!(
 //        r#"
 //     ║   
 // ════ ┄┄┄ 
 //     ║   
 // "#
 //    );
 //    print!("\x1b[0m");
 //    return;
    let mut buffer = PixelBuffer::default();

    let scale = 100;
    let noise = NoiseMap::fbm(thread_rng().gen::<u32>())
        .size(500, 500)
        .scale(10.)
        .bounds(-1., 1.)
        .build();

    print!("\x1b[?25h");

    let frame_rate: f32 = 1. / 12.; // 12 frames per second
    for i in 0..scale {
        let start = Instant::now();
        print!("\x1b[H");
        for h in 0..buffer.height() {
            for w in 0..buffer.width() {
                let sample = noise.get(i + w, i + h);
                let mut n = (11. * sample) as i16;

                if n < -11 || n > 11 {
                    panic!("Invalid noise value: {} = {}", sample, n);
                }
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
        print!("{}", buffer.render().unwrap());
        io::stdout().flush().unwrap();

        let dt = (1000. * frame_rate) - start.elapsed().as_millis() as f32;
        if dt > 0. {
            sleep(Duration::from_millis(dt as u64));
            // update(frame_rate).unwrap();
        } else if dt > 0. {
            // update(dt).unwrap();
            panic!("Took too long");
        }
    }

    print!("\x1b[0m");
    // print!("\x1b[2J\x1b[H");
    print!("\x1b[?25l");
    println!("({}, {})", buffer.width(), buffer.height());
}
