pub mod buffer;
pub mod data;
mod noise_map;

pub mod color;
pub use noise_map::NoiseMap;

pub fn get_term_size() -> (usize, usize) {
    let termsize::Size { rows, cols } = termsize::get().unwrap();
    (cols as usize, rows as usize)
}
