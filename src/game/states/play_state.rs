use std::collections::VecDeque;
use game::{Event, State, Renderer};

// TODO: Pass when constructing the state
const BOARD_WIDTH: usize = 20;
const BOARD_HEIGHT: usize = 8;

pub struct PlayState {
    player: Actor,
    sea_enemies: Vec<Actor>,
    land_enemies: Vec<Actor>,
    board: Vec<Vec<Field>>,
}

struct Actor {
    x: i16,
    y: i16,
    dx: i16,
    dy: i16,
}

#[derive(Clone,PartialEq)]
enum Field {
    Land,
    Sea,
    DeepSea,
    Sand,
}

fn flood_fill(board: &mut Vec<Vec<Field>>, position: (i16, i16)) {
    let mut q = VecDeque::new();
    q.push_back(position);
    while !q.is_empty() {
        let (x, y) = q.pop_front().unwrap();
        if board[y as usize][x as usize] == Field::Sea {
            board[y as usize][x as usize] = Field::DeepSea;
            q.push_back((x, y - 1));
            q.push_back((x, y + 1));
            q.push_back((x - 1, y));
            q.push_back((x + 1, y));
        }
    }


}

fn fill(board: &mut Vec<Vec<Field>>, enemies: &Vec<Actor>) {
    for e in enemies {
        flood_fill(board, (e.x, e.y));
    }

    for row in board.iter_mut() {
        for field in row.iter_mut() {
            if *field == Field::DeepSea {
                *field = Field::Sea;
            } else if *field == Field::Sea || *field == Field::Sand {
                *field = Field::Land;
            }
        }
    }
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
            player: Actor {
                x: 0, y: 0,
                dx: 0, dy: 0,
            },
            sea_enemies: vec![
                Actor { x: 2, y: 2, dx: 1, dy: 1 },
            ],
            land_enemies: vec![
                Actor { x: BOARD_WIDTH as i16 / 2, y: 0, dx: 1, dy: 1 },
            ],
            board,
        }
    }

    fn move_player(&mut self) -> Result<(), ()> {
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

                    fill(&mut self.board, &self.sea_enemies);
                } else if let Field::Sand = self.board[y as usize][x as usize] {
                    return Err(());
                }
            }

            player.x = x;
            player.y = y;
        }

        Ok(())
    }

    fn move_sea_enemies(&mut self) -> Result<(), ()> {
        for enemy in self.sea_enemies.iter_mut() {
            let (x, y) = (enemy.x, enemy.y);
            let (mut dx, mut dy) = (enemy.dx, enemy.dy);

            // Land in my horizontal direction?
            if let Field::Land = self.board[y as usize][(x + dx) as usize] {
                dx = -dx;
            }

            // Land in my vertical direction?
            if let Field::Land = self.board[(y + dy) as usize][x as usize] {
                dy = -dy;
            }

            // Land exactly in diagonal?
            if let Field::Land = self.board[(y + dy) as usize][(x + dx) as usize] {
                dx = -dx;
                dy = -dy;
            }

            // Check for collision with player
            if x + dx == self.player.x && y + dy == self.player.y ||
                x == self.player.x && y + dy == self.player.y ||
                x + dx == self.player.x && y == self.player.y {
                return Err(());
            }

            // Check for collision with sand
            if self.board[(y + dy) as usize][(x + dx) as usize] == Field::Sand ||
                self.board[(y + dy) as usize][x as usize] == Field::Sand ||
                self.board[y as usize][(x + dx) as usize] == Field::Sand {
                return Err(());
            }

            enemy.dx = dx;
            enemy.dy = dy;
            enemy.x = x + dx;
            enemy.y = y + dy;
        }

        Ok(())
    }

    fn move_land_enemies(&mut self) -> Result<(), ()> {
        for enemy in self.land_enemies.iter_mut() {
            let (x, y) = (enemy.x, enemy.y);
            let (mut dx, mut dy) = (enemy.dx, enemy.dy);

            // Land or edge in my horizontal direction?
            if x + dx < 0 || x + dx >= BOARD_WIDTH as i16 || self.board[y as usize][(x + dx) as usize] != Field::Land {
                dx = -dx;
            }

            // Land or edge in my vertical direction?
            if y + dy < 0 || y + dy >= BOARD_HEIGHT as i16 || self.board[(y + dy) as usize][x as usize] != Field::Land {
                dy = -dy;
            }

            // Land exactly in diagonal?
            if self.board[(y + dy) as usize][(x + dx) as usize] != Field::Land {
                dx = -dx;
                dy = -dy;
            }

            // Check for collision
            if x + dx == self.player.x && y + dy == self.player.y ||
                x == self.player.x && y + dy == self.player.y ||
                x + dx == self.player.x && y == self.player.y {
                return Err(());
            }

            enemy.x = x + dx;
            enemy.y = y + dy;
            enemy.dx = dx;
            enemy.dy = dy;
        }

        Ok(())
    }

    fn move_actors(&mut self) -> Result<(), ()> {
        self.move_player()?;
        self.move_sea_enemies()?;
        self.move_land_enemies()?;

        Ok(())
    }
}

impl State for PlayState {
    fn update(&mut self) -> Option<Box<State>> {
        if self.move_actors().is_ok() {
            None
        } else {
            Some(Box::new(super::GameOverState {}))
        }
    }

    fn render(&mut self, renderer: &mut Renderer) {
        for (y, row) in self.board.iter().enumerate() {
            for (x, field) in row.iter().enumerate() {
                let c = match field {
                    &Field::Land => '█',
                    &Field::Sea => '░',
                    &Field::Sand => '▒',
                    _ => '?',
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

        for e in self.sea_enemies.iter() {
            renderer.put_cell(e.x as u16, e.y as u16, 'S');
        }

        for e in self.land_enemies.iter() {
            renderer.put_cell(e.x as u16, e.y as u16, 'L');
        }
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
