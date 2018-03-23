mod models;

use rand::{thread_rng, Rng};
use game::{Event, State, Renderer};
use self::models::{Board, Field};

// TODO: Pass when constructing the state
const BOARD_WIDTH: usize = 32;
const BOARD_HEIGHT: usize = 12;

#[derive(Clone, PartialEq)]
pub struct Position {
    x: i16,
    y: i16,
}

impl Position {
    // TODO: Implement as addition with `+`
    fn moved_to(&self, direction: &Direction) -> Position {
        let deltas = direction.deltas();
        let x = self.x + deltas.0;
        let y = self.y + deltas.1;
        Position { x, y }
    }
}

#[derive(Copy, Clone)]
pub enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    SouthEast,
    NorthWest,
    SouthWest,
    None,
}

impl Direction {
    fn deltas(self) -> (i16, i16) {
        match self {
            Direction::North     => ( 0, -1),
            Direction::South     => ( 0,  1),
            Direction::East      => ( 1,  0),
            Direction::West      => (-1,  0),
            Direction::NorthEast => ( 1, -1),
            Direction::SouthEast => ( 1,  1),
            Direction::NorthWest => (-1, -1),
            Direction::SouthWest => (-1,  1),
            Direction::None      => ( 0,  0),
        }
    }

    fn flipped_x(self) -> Self {
        match self {
            Direction::North     => Direction::North,
            Direction::South     => Direction::South,
            Direction::East      => Direction::West,
            Direction::West      => Direction::East,
            Direction::NorthEast => Direction::NorthWest,
            Direction::SouthEast => Direction::SouthWest,
            Direction::NorthWest => Direction::NorthEast,
            Direction::SouthWest => Direction::SouthEast,
            Direction::None      => Direction::None,
        }
    }

    fn flipped_y(self) -> Self {
        match self {
            Direction::North     => Direction::South,
            Direction::South     => Direction::North,
            Direction::East      => Direction::East,
            Direction::West      => Direction::West,
            Direction::NorthEast => Direction::SouthEast,
            Direction::SouthEast => Direction::NorthEast,
            Direction::NorthWest => Direction::SouthWest,
            Direction::SouthWest => Direction::NorthWest,
            Direction::None      => Direction::None,
        }
    }

    fn horizontal(self) -> Self {
        match self {
            Direction::NorthEast => Direction::East,
            Direction::SouthEast => Direction::East,
            Direction::NorthWest => Direction::West,
            Direction::SouthWest => Direction::West,
            _                    => Direction::None,
        }
    }

    fn vertical(self) -> Self {
        match self {
            Direction::NorthEast => Direction::North,
            Direction::SouthEast => Direction::South,
            Direction::NorthWest => Direction::North,
            Direction::SouthWest => Direction::South,
            _                    => Direction::None,
        }
    }
}

fn random_diagonal() -> Direction {
    *thread_rng().choose(&[
        Direction::NorthEast,
        Direction::SouthEast,
        Direction::NorthWest,
        Direction::SouthWest,
    ]).unwrap()
}

struct Actor {
    position: Position,
    direction: Direction,
}

pub struct PlayState {
    player: Actor,
    sea_enemies: Vec<Actor>,
    land_enemies: Vec<Actor>,
    board: Board,
}

impl PlayState {
    pub fn new() -> PlayState {
        let board = Board::new(BOARD_WIDTH, BOARD_HEIGHT);

        PlayState {
            player: Actor {
                position: Position { x: 0, y: 0 },
                direction: Direction::None,
            },
            sea_enemies: vec![
                Actor {
                    position: board.random_position_of_type(Field::Sea),
                    direction: random_diagonal(),
                },
            ],
            land_enemies: vec![
                Actor {
                    position: Position {
                        x: BOARD_WIDTH as i16 / 2,
                        y: BOARD_HEIGHT as i16 - 2,
                    },
                    direction: random_diagonal(),
                },
            ],
            board,
        }
    }

    fn move_player(&mut self) -> Result<(), ()> {
        let player = &mut self.player;
        let pos = player.position.moved_to(&player.direction);

        if !self.board.within_bounds(&pos) {
            player.direction = Direction::None;
        } else {
            if self.board[&player.position] == Field::Sea {
                self.board[&player.position] = Field::Sand;

                if self.board[&pos] == Field::Land {
                    player.direction = Direction::None;

                    // TODO: This can be part of 'fill'
                    for row in self.board.rows_mut() {
                        for field in row.iter_mut() {
                            if *field == Field::Sand {
                                *field = Field::Land;
                            }
                        }
                    }

                    let enemy_positions: Vec<&Position> =
                        self.sea_enemies.iter().map(|e| &e.position).collect();
                    self.board.fill(&enemy_positions);
                } else if self.board[&pos] == Field::Sand {
                    return Err(());
                }
            }

            player.position = pos;
        }

        Ok(())
    }

    fn move_sea_enemies(&mut self) -> Result<(), ()> {
        for enemy in self.sea_enemies.iter_mut() {
            let position = enemy.position.clone();
            let mut direction = enemy.direction.clone();

            // Land in my horizontal direction?
            if self.board[&position.moved_to(&direction.horizontal())] == Field::Land {
                direction = direction.flipped_x();
            }

            // Land in my vertical direction?
            if self.board[&position.moved_to(&direction.vertical())] == Field::Land {
                direction = direction.flipped_y();
            }

            // Land exactly in diagonal?
            // if let Field::Land = self.board.fields[(y + dy) as usize][(x + dx) as usize] {
            if self.board[&position.moved_to(&direction)] == Field::Land {
                direction = direction.flipped_x().flipped_y();
            }

            // Check for collision with player
            if position.moved_to(&direction) == self.player.position ||
                position.moved_to(&direction.horizontal()) == self.player.position ||
                position.moved_to(&direction.vertical()) == self.player.position {
                return Err(());
            }

            // Check for collision with sand
            if self.board[&position.moved_to(&direction)] == Field::Sand ||
                self.board[&position.moved_to(&direction.horizontal())] == Field::Sand ||
                self.board[&position.moved_to(&direction.vertical())] == Field::Sand {
                return Err(());
            }

            enemy.position = position.moved_to(&direction);
            enemy.direction = direction;
        }

        Ok(())
    }

    // fn move_land_enemies(&mut self) -> Result<(), ()> {
    //     for enemy in self.land_enemies.iter_mut() {
    //         let (x, y) = (enemy.position.x, enemy.position.y);
    //         let (mut dx, mut dy) = (enemy.dx, enemy.dy);
    //
    //         // Land or edge in my horizontal direction?
    //         if x + dx < 0 || x + dx >= BOARD_WIDTH as i16 || self.board.fields[y as usize][(x + dx) as usize] != Field::Land {
    //             dx = -dx;
    //         }
    //
    //         // Land or edge in my vertical direction?
    //         if y + dy < 0 || y + dy >= BOARD_HEIGHT as i16 || self.board.fields[(y + dy) as usize][x as usize] != Field::Land {
    //             dy = -dy;
    //         }
    //
    //         // Land exactly in diagonal?
    //         if self.board.fields[(y + dy) as usize][(x + dx) as usize] != Field::Land {
    //             dx = -dx;
    //             dy = -dy;
    //         }
    //
    //         // Check for collision
    //         if x + dx == self.player.position.x && y + dy == self.player.position.y ||
    //             x == self.player.position.x && y + dy == self.player.position.y ||
    //             x + dx == self.player.position.x && y == self.player.position.y {
    //             return Err(());
    //         }
    //
    //         enemy.position.x = x + dx;
    //         enemy.position.y = y + dy;
    //         enemy.dx = dx;
    //         enemy.dy = dy;
    //     }
    //
    //     Ok(())
    // }

    fn move_actors(&mut self) -> Result<(), ()> {
        self.move_player()?;
        self.move_sea_enemies()?;
        // self.move_land_enemies()?;

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
        for (y, row) in self.board.rows().enumerate() {
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
            renderer.put_cell(x as u16, self.board.rows().len() as u16, c);
        }
    }

    fn handle_event(&mut self, event: Event) -> Option<Box<State>> {
        match event {
            Event::Up => {
                self.player.direction = Direction::North;
            },
            Event::Down => {
                self.player.direction = Direction::South;
            },
            Event::Left => {
                self.player.direction = Direction::West;
            },
            Event::Right => {
                self.player.direction = Direction::East;
            },
            _ => {},
        }

        None
    }
}
