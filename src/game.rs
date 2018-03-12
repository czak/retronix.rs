use std::collections::VecDeque;

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 5;

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
        let x = self.x + self.dx;
        let y = self.y + self.dy;

        if x < 0 || x >= BOARD_WIDTH as i16 ||
            y < 0 || y >= BOARD_HEIGHT as i16 {
            self.dx = 0;
            self.dy = 0;
        } else {
            self.x = x;
            self.y = y;
        }
    }
}

#[derive(Clone)]
enum Field {
    Land,
}

pub struct Game {
    player: Player,
    board: Vec<Vec<Field>>,
    events: VecDeque<Event>,
}

impl Game {
    pub fn render<R: Renderer>(&mut self, renderer: &mut R) {
        for (y, row) in self.board.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                renderer.put_cell(
                    x as u16,
                    y as u16,
                    '█'
                );
            }
        }

        renderer.put_cell(
            self.player.x as u16,
            self.player.y as u16,
            'x'
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
            x: 0, y: 0,
            dx: 0, dy: 0,
        },
        board: vec![vec![Field::Land; BOARD_WIDTH]; BOARD_HEIGHT],
        events: VecDeque::new(),
    }
}
