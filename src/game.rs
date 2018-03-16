use std::collections::VecDeque;

const BOARD_WIDTH: usize = 20;
const BOARD_HEIGHT: usize = 8;

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

#[derive(Clone)]
enum Field {
    Land,
    Sea,
    Sand,
}

pub struct Game {
    player: Player,
    board: Vec<Vec<Field>>,
    events: VecDeque<Event>,
}

impl Game {
    pub fn render<R: Renderer>(&mut self, renderer: &mut R) {
        for (y, row) in self.board.iter().enumerate() {
            for (x, field) in row.iter().enumerate() {
                let c = match field {
                    &Field::Land => '█',
                    &Field::Sea => '░',
                    &Field::Sand => '▒',
                };
                renderer.put_cell(
                    x as u16,
                    y as u16,
                    c
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
        self.move_player();
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

    fn move_player(&mut self) {
        let player = &mut self.player;
        let x = player.x + player.dx;
        let y = player.y + player.dy;

        if x < 0 || x >= BOARD_WIDTH as i16 ||
            y < 0 || y >= BOARD_HEIGHT as i16 {
            player.dx = 0;
            player.dy = 0;
        } else {
            if let Field::Sea = self.board[player.y as usize][player.x as usize] {
                self.board[player.y as usize][player.x as usize] = Field::Sand;

                if let Field::Land = self.board[y as usize][x as usize] {
                    player.dx = 0;
                    player.dy = 0;

                    for row in self.board.iter_mut() {
                        for field in row.iter_mut() {
                            if let &mut Field::Sand = field {
                                *field = Field::Land;
                            }
                        }
                    }
                }
            }

            player.x = x;
            player.y = y;
        }
    }
}

pub fn init() -> Game {
    let mut board = vec![vec![Field::Sea; BOARD_WIDTH]; BOARD_HEIGHT];

    for (y, row) in board.iter_mut().enumerate() {
        for (x, field) in row.iter_mut().enumerate() {
            if x < 2 || x >= BOARD_WIDTH - 2 || y < 2 || y >= BOARD_HEIGHT -2 {
                *field = Field::Land;
            }
        }
    }

    Game {
        player: Player {
            x: 0, y: 0,
            dx: 0, dy: 0,
        },
        board,
        events: VecDeque::new(),
    }
}
