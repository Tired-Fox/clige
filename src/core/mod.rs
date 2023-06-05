pub mod buffer;
pub mod data;

pub mod color;

pub fn get_term_size() -> (usize, usize) {
    let termsize::Size { rows, cols } = termsize::get().unwrap();
    (cols as usize, rows as usize)
}
