use super::position;
use crate::elements::{Canvas, View, Viewable};
use std::io::{self, Write};

pub fn render(canvas: &mut Canvas) {
    position(canvas.x, canvas.y);
    canvas.reset();

    let cview = canvas.view();
    for child in canvas.children.iter() {
        match &*child.borrow() {
            View::Text(text) => {
                let text = &*text.borrow();
                let (x, y) = text.position();
                let text_view = text.view();

                for (h, line) in text_view.iter().enumerate() {
                    for (w, char) in line.iter().enumerate() {
                        cview[h + y][w + x].replace(char.borrow().clone());
                    }
                }
            }
            View::Canvas(_self) => {
                // TODO: implement nested canvas
            }
        }
    }
}

pub fn draw(canvas: &mut Canvas) {
    render(canvas);
    print!("{}", canvas);
    io::stdout().flush().unwrap();
}
