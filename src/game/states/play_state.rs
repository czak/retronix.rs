use game::{Event, State, Renderer};

// TODO: Pass when constructing the state
const BOARD_WIDTH: usize = 20;
const BOARD_HEIGHT: usize = 8;

pub struct PlayState {
    player: Player,
    board: Vec<Vec<Field>>,
}

struct Player {
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

impl PlayState {
    pub fn new() -> PlayState {
        let mut board = vec![vec![Field::Sea; BOARD_WIDTH]; BOARD_HEIGHT];

        for (y, row) in board.iter_mut().enumerate() {
            for (x, field) in row.iter_mut().enumerate() {
                if x < 2 || x >= BOARD_WIDTH - 2 || y < 2 || y >= BOARD_HEIGHT -2 {
                    *field = Field::Land;
                }
            }
        }

        PlayState {
            player: Player {
                x: 0, y: 0,
                dx: 0, dy: 0,
            },
            board,
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

impl State for PlayState {
    fn update(&mut self) -> Option<Box<State>> {
        self.move_player();

        None
    }

    fn render(&mut self, renderer: &mut Renderer) {
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

    fn handle_event(&mut self, event: Event) -> Option<Box<State>> {
        match event {
            Event::Up => {
                self.player.dx = 0;
                self.player.dy = -1;
            },
            Event::Down => {
                self.player.dx = 0;
                self.player.dy = 1;
            },
            Event::Left => {
                self.player.dx = -1;
                self.player.dy = 0;
            },
            Event::Right => {
                self.player.dx = 1;
                self.player.dy = 0;
            },
            _ => {},
        }

        None
    }
}
