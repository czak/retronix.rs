mod models;

use rand::{thread_rng, Rng};
use game::{Event, State, Renderer};
use self::models::{Board, Field};

// TODO: Pass when constructing the state
const BOARD_WIDTH: usize = 32;
const BOARD_HEIGHT: usize = 12;

pub struct PlayState {
    player: Actor,
    sea_enemies: Vec<Actor>,
    land_enemies: Vec<Actor>,
    board: Board,
}

pub struct Position {
    x: i16,
    y: i16,
}

impl Position {
    fn moved_to(&self, dx: i16, dy: i16) -> Position {
        let x = self.x + dx;
        let y = self.y + dy;
        Position { x, y }
    }
}

struct Actor {
    position: Position,
    dx: i16,
    dy: i16,
}

impl PlayState {
    pub fn new() -> PlayState {
        let board = Board::new(BOARD_WIDTH, BOARD_HEIGHT);

        let choices = [-1, 1];
        let mut rng = thread_rng();

        PlayState {
            player: Actor {
                position: Position { x: 0, y: 0 },
                dx: 0, dy: 0,
            },
            sea_enemies: vec![
                Actor {
                    position: board.random_position_of_type(Field::Sea),
                    dx: *rng.choose(&choices).unwrap(),
                    dy: *rng.choose(&choices).unwrap(),
                },
            ],
            land_enemies: vec![
                Actor {
                    position: Position {
                        x: BOARD_WIDTH as i16 / 2,
                        y: BOARD_HEIGHT as i16 - 2,
                    },
                    dx: *rng.choose(&choices).unwrap(),
                    dy: *rng.choose(&choices).unwrap(),
                },
            ],
            board,
        }
    }

    fn move_player(&mut self) -> Result<(), ()> {
        let player = &mut self.player;
        let pos = player.position.moved_to(player.dx, player.dy);

        if pos.x < 0 || pos.x >= BOARD_WIDTH as i16 ||
            pos.y < 0 || pos.y >= BOARD_HEIGHT as i16 {
            player.dx = 0;
            player.dy = 0;
        } else {
            if let Field::Sea = self.board.fields[player.position.y as usize][player.position.x as usize] {
                self.board.fields[player.position.y as usize][player.position.x as usize] = Field::Sand;

                if let Field::Land = self.board.fields[pos.y as usize][pos.x as usize] {
                    player.dx = 0;
                    player.dy = 0;

                    for row in self.board.fields.iter_mut() {
                        for field in row.iter_mut() {
                            if let &mut Field::Sand = field {
                                *field = Field::Land;
                            }
                        }
                    }

                    self.board.fill(&self.sea_enemies.iter().map(|e| (e.position.x, e.position.y)).collect());
                } else if let Field::Sand = self.board.fields[pos.y as usize][pos.x as usize] {
                    return Err(());
                }
            }

            player.position = pos;
        }

        Ok(())
    }

    fn move_sea_enemies(&mut self) -> Result<(), ()> {
        for enemy in self.sea_enemies.iter_mut() {
            let (x, y) = (enemy.position.x, enemy.position.y);
            let (mut dx, mut dy) = (enemy.dx, enemy.dy);

            // Land in my horizontal direction?
            if let Field::Land = self.board.fields[y as usize][(x + dx) as usize] {
                dx = -dx;
            }

            // Land in my vertical direction?
            if let Field::Land = self.board.fields[(y + dy) as usize][x as usize] {
                dy = -dy;
            }

            // Land exactly in diagonal?
            if let Field::Land = self.board.fields[(y + dy) as usize][(x + dx) as usize] {
                dx = -dx;
                dy = -dy;
            }

            // Check for collision with player
            if x + dx == self.player.position.x && y + dy == self.player.position.y ||
                x == self.player.position.x && y + dy == self.player.position.y ||
                x + dx == self.player.position.x && y == self.player.position.y {
                return Err(());
            }

            // Check for collision with sand
            if self.board.fields[(y + dy) as usize][(x + dx) as usize] == Field::Sand ||
                self.board.fields[(y + dy) as usize][x as usize] == Field::Sand ||
                self.board.fields[y as usize][(x + dx) as usize] == Field::Sand {
                return Err(());
            }

            enemy.dx = dx;
            enemy.dy = dy;
            enemy.position.x = x + dx;
            enemy.position.y = y + dy;
        }

        Ok(())
    }

    fn move_land_enemies(&mut self) -> Result<(), ()> {
        for enemy in self.land_enemies.iter_mut() {
            let (x, y) = (enemy.position.x, enemy.position.y);
            let (mut dx, mut dy) = (enemy.dx, enemy.dy);

            // Land or edge in my horizontal direction?
            if x + dx < 0 || x + dx >= BOARD_WIDTH as i16 || self.board.fields[y as usize][(x + dx) as usize] != Field::Land {
                dx = -dx;
            }

            // Land or edge in my vertical direction?
            if y + dy < 0 || y + dy >= BOARD_HEIGHT as i16 || self.board.fields[(y + dy) as usize][x as usize] != Field::Land {
                dy = -dy;
            }

            // Land exactly in diagonal?
            if self.board.fields[(y + dy) as usize][(x + dx) as usize] != Field::Land {
                dx = -dx;
                dy = -dy;
            }

            // Check for collision
            if x + dx == self.player.position.x && y + dy == self.player.position.y ||
                x == self.player.position.x && y + dy == self.player.position.y ||
                x + dx == self.player.position.x && y == self.player.position.y {
                return Err(());
            }

            enemy.position.x = x + dx;
            enemy.position.y = y + dy;
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
        for (y, row) in self.board.fields.iter().enumerate() {
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
            self.player.position.x as u16,
            self.player.position.y as u16,
            'x'
        );

        for e in self.sea_enemies.iter() {
            renderer.put_cell(e.position.x as u16, e.position.y as u16, 'S');
        }

        for e in self.land_enemies.iter() {
            renderer.put_cell(e.position.x as u16, e.position.y as u16, 'L');
        }

        let score = "Score: 0 Xn: 3 Full: 0% Time: 90";
        for (x, c) in score.chars().enumerate() {
            renderer.put_cell(x as u16, self.board.fields.len() as u16, c);
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
