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
use clige::gen::{NoiseMap, Random};
use rand::Rng;
//sige

fn main() {
    // let mut rng = Random::new();

    // let mut rng = Random::from("Some Seed goes here 2109@");
    // let mut rng = Random::from("6100374812663787999");
    let mut rng = Random::from(6100374812663787999);
    let noise = NoiseMap::perlin(rng.seed())
        .step(0.15, 0.15)
        .bounds(-2., 2.)
        .build();

    println!("{}", rng.seed());
    println!(
        "{} {}",
        rng.generator().gen_range(0..100),
        noise.get(600, 100_052)
    )
}
