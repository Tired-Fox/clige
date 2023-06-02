//! Can subdivide at random locations
//!     Choose random position along road to divide
//!
//! Can use L-Systems to generate nodes and use proximity calculations to generate intersections
//! Nodes:
//!     - Large main road
//!     - Smaller side road
//!     - Alley way
//! Random Chance:
//!     - Instruction Set
//!     - Angle
//!     - Length
//! Manipulation (Move points to look better):
//!     - Intersect another road
//!     - Near another road
//!
extern crate clige;

use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};

use clige::core::{buffer::{PixelBuffer, Buffer}, data::Pixel, NoiseMap};
use rand::{thread_rng, SeedableRng, prelude::*};
use rand_chacha::ChaCha8Rng;


fn main () {

    let input: String = String::from("6100374812663787999");
    // let input: String = String::from("Some Seed goes here 2109@");

    let seed = match input.parse::<i64>() {
        Ok(seed) => {
            seed
        }
        Err(_) => {
            let mut hasher = DefaultHasher::new();
            input.hash(&mut hasher);
            hasher.finish() as i64
        }

    };

    println!("{}", seed);

    let mut rng = ChaCha8Rng::seed_from_u64(seed as u64);
    let noise = NoiseMap::perlin(seed as u32)
        .step(0.15, 0.15)
        .bounds(-2., 2.)
        .build();

    // let map = PixelBuffer::default();
    println!("{} {}", rng.gen_range(0..100), noise.get(600, 100_052))
}
