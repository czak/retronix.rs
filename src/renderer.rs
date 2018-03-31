pub trait Renderer {
    fn put_cell(&mut self, x: u16, y: u16, c: char);
}
