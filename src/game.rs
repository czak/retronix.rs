pub trait Renderer {
    fn put_cell(&mut self, x: u16, y: u16, c: char);
}

pub struct Game {
    x: u16,
    y: u16,
}

impl Game {
    pub fn render<R: Renderer>(&mut self, renderer: &mut R) {
        renderer.put_cell(self.x, self.y, '█');
    }

    pub fn update(&mut self) {
        self.x += 1;
        self.y += 1;
    }
}

pub fn init() -> Game {
    Game {
        x: 2,
        y: 2,
    }
}
