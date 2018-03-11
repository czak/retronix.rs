pub trait Renderer {
    fn put_cell(&mut self, x: u16, y: u16, c: char);
}

pub struct Game {}

impl Game {
    pub fn render<R: Renderer>(&mut self, renderer: &mut R) {
        renderer.put_cell(2, 2, 'X');
    }
}

pub fn init() -> Game {
    Game {}
}
