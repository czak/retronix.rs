#[derive(Copy, Clone)]
pub enum Color {
    White,
    Cyan,
    Magenta,
}

pub trait Renderer {
    fn put_cell(&mut self, x: u16, y: u16, c: char, color: Color);
}
