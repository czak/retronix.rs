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

struct Actor {
    x: i16,
    y: i16,
    dx: i16,
    dy: i16,
}

impl PlayState {
    pub fn new() -> PlayState {
        let board = Board::new(BOARD_WIDTH, BOARD_HEIGHT);

        let choices = [-1, 1];
        let mut rng = thread_rng();

        let random_sea_position = board.random_position_of_type(Field::Sea);

        PlayState {
            player: Actor {
                x: 0, y: 0,
                dx: 0, dy: 0,
            },
            sea_enemies: vec![
                Actor {
                    x: random_sea_position.0,
                    y: random_sea_position.1,
                    dx: *rng.choose(&choices).unwrap(),
                    dy: *rng.choose(&choices).unwrap(),
                },
            ],
            land_enemies: vec![
                Actor {
                    x: BOARD_WIDTH as i16 / 2,
                    y: BOARD_HEIGHT as i16 - 2,
                    dx: *rng.choose(&choices).unwrap(),
                    dy: *rng.choose(&choices).unwrap(),
                },
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
            if let Field::Sea = self.board.fields[player.y as usize][player.x as usize] {
                self.board.fields[player.y as usize][player.x as usize] = Field::Sand;

                if let Field::Land = self.board.fields[y as usize][x as usize] {
                    player.dx = 0;
                    player.dy = 0;

                    for row in self.board.fields.iter_mut() {
                        for field in row.iter_mut() {
                            if let &mut Field::Sand = field {
                                *field = Field::Land;
                            }
                        }
                    }

                    self.board.fill(&self.sea_enemies.iter().map(|e| (e.x, e.y)).collect());
                } else if let Field::Sand = self.board.fields[y as usize][x as usize] {
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
            if x + dx == self.player.x && y + dy == self.player.y ||
                x == self.player.x && y + dy == self.player.y ||
                x + dx == self.player.x && y == self.player.y {
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
