use std::collections::VecDeque;

pub enum Event {
    Tick,
    Quit,
    Up,
    Down,
    Left,
    Right,
}

pub trait Renderer {
    fn put_cell(&mut self, x: u16, y: u16, c: char);
}

pub struct Player {
    x: i16,
    y: i16,
    dx: i16,
    dy: i16,
}

impl Player {
    fn animate(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }
}

pub struct Game {
    player: Player,
    events: VecDeque<Event>,
}

impl Game {
    pub fn render<R: Renderer>(&mut self, renderer: &mut R) {
        renderer.put_cell(
            self.player.x as u16,
            self.player.y as u16,
            '█'
        );
    }

    pub fn update(&mut self) {
        self.player.animate();
    }

    pub fn push_event(&mut self, e: Event) {
        self.events.push_back(e);
    }

    pub fn handle_event(&mut self) {
        match self.events.pop_front() {
            Some(Event::Up) => {
                self.player.dx = 0;
                self.player.dy = -1;
            },
            Some(Event::Down) => {
                self.player.dx = 0;
                self.player.dy = 1;
            },
            Some(Event::Left) => {
                self.player.dx = -1;
                self.player.dy = 0;
            },
            Some(Event::Right) => {
                self.player.dx = 1;
                self.player.dy = 0;
            },
            _ => {},
        }
    }
}

pub fn init() -> Game {
    Game {
        player: Player {
            x: 1, y: 1,
            dx: 0, dy: 0,
        },
        events: VecDeque::new(),
    }
}
