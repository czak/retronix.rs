mod models;

use rand::{thread_rng, Rng};
use game::{Event, State, Renderer, Transition};
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
    fn moved_to(&self, direction: &Direction) -> Position {
        Position {
            x: self.x + direction.dx,
            y: self.y + direction.dy,
        }
    }
}

#[derive(Clone)]
pub struct Direction {
    dx: i16,
    dy: i16,
}

impl Direction {
    const NORTH:     Direction = Direction { dx: 0, dy: -1 };
    const SOUTH:     Direction = Direction { dx: 0, dy: 1 };
    const EAST:      Direction = Direction { dx: 1, dy: 0 };
    const WEST:      Direction = Direction { dx: -1, dy: 0 };
    const NORTHEAST: Direction = Direction { dx: 1, dy: -1 };
    const SOUTHEAST: Direction = Direction { dx: 1, dy: 1 };
    const NORTHWEST: Direction = Direction { dx: -1, dy: -1 };
    const SOUTHWEST: Direction = Direction { dx: -1, dy: 1 };
    const NONE:      Direction = Direction { dx: 0, dy: 0 };

    fn horizontal(&self) -> Direction {
        Direction {
            dx: self.dx,
            dy: 0,
        }
    }

    fn vertical(&self) -> Direction {
        Direction {
            dx: 0,
            dy: self.dy,
        }
    }

    fn flipped_x(&self) -> Direction {
        Direction {
            dx: -self.dx,
            dy: self.dy,
        }
    }

    fn flipped_y(&self) -> Direction {
        Direction {
            dx: self.dx,
            dy: -self.dy,
        }
    }
}

fn random_diagonal() -> Direction {
    static DIAGONALS: [Direction; 4] = [
        Direction::NORTHEAST,
        Direction::NORTHWEST,
        Direction::SOUTHEAST,
        Direction::SOUTHWEST,
    ];
    thread_rng().choose(&DIAGONALS).unwrap().clone()
}

struct Player {
    position: Position,
    direction: Direction,
}

struct Enemy {
    position: Position,
    direction: Direction,
}

impl Enemy {
    /// Array of positions where the enemy hits an object
    fn hit_positions(&self) -> [Position; 3] {
        [
            self.position.moved_to(&self.direction),
            self.position.moved_to(&self.direction.horizontal()),
            self.position.moved_to(&self.direction.vertical()),
        ]
    }

    fn hit_check<F>(&self, position_test: F) -> bool
        where F: Fn(&Position) -> bool
    {
        self.hit_positions().iter().any(position_test)
    }
}

enum Delay {
    Death(u32),
    NextLevel(u32),
    None,
}

pub struct PlayState {
    player: Player,
    sea_enemies: Vec<Enemy>,
    land_enemies: Vec<Enemy>,
    board: Board,
    level: u32,
    score: u32,
    lives: u32,
    delay: Delay,
}

impl PlayState {
    pub fn new(level: u32, score: u32, lives: u32) -> PlayState {
        let board = Board::new(BOARD_WIDTH, BOARD_HEIGHT);

        let mut sea_enemies = vec![];
        for _ in 0..level {
            sea_enemies.push(Enemy {
                position: board.random_position_of_type(Field::Sea),
                direction: random_diagonal(),
            });
        }

        PlayState {
            player: Player {
                position: Position {
                    x: BOARD_WIDTH as i16 / 2,
                    y: 0,
                },
                direction: Direction::NONE,
            },
            sea_enemies,
            land_enemies: vec![
                Enemy {
                    position: Position {
                        x: BOARD_WIDTH as i16 / 2,
                        y: BOARD_HEIGHT as i16 - 2,
                    },
                    direction: random_diagonal(),
                },
            ],
            board,
            level,
            score,
            lives,
            delay: Delay::None,
        }
    }

    fn move_player(&mut self) {
        let player = &mut self.player;
        let pos = player.position.moved_to(&player.direction);

        if !self.board.within_bounds(&pos) {
            player.direction = Direction::NONE;
        } else {
            if self.board[&player.position] == Field::Sea {
                self.board[&player.position] = Field::Sand;

                if self.board[&pos] == Field::Land {
                    player.direction = Direction::NONE;

                    let enemy_positions: Vec<&Position> =
                        self.sea_enemies.iter().map(|e| &e.position).collect();
                    self.score += self.board.fill(&enemy_positions);
                }
            }

            player.position = pos;
        }
    }

    fn bounce_sea_enemies(&mut self) {
        for enemy in self.sea_enemies.iter_mut() {
            // Land in my horizontal direction?
            if self.board[&enemy.position.moved_to(&enemy.direction.horizontal())] == Field::Land {
                enemy.direction = enemy.direction.flipped_x();
            }

            // Land in my vertical direction?
            if self.board[&enemy.position.moved_to(&enemy.direction.vertical())] == Field::Land {
                enemy.direction = enemy.direction.flipped_y();
            }

            // Land exactly in diagonal?
            if self.board[&enemy.position.moved_to(&enemy.direction)] == Field::Land {
                enemy.direction = enemy.direction.flipped_x().flipped_y();
            }
        }
    }

    fn move_sea_enemies(&mut self) {
        for enemy in self.sea_enemies.iter_mut() {
            enemy.position = enemy.position.moved_to(&enemy.direction);
        }
    }

    fn bounce_land_enemies(&mut self) {
        for enemy in self.land_enemies.iter_mut() {
            // Land in my horizontal direction?
            let pos = enemy.position.moved_to(&enemy.direction.horizontal());
            if !self.board.within_bounds(&pos) || self.board[&pos] != Field::Land {
                enemy.direction = enemy.direction.flipped_x();
            }

            // Land in my vertical direction?
            let pos = enemy.position.moved_to(&enemy.direction.vertical());
            if !self.board.within_bounds(&pos) || self.board[&pos] != Field::Land {
                enemy.direction = enemy.direction.flipped_y();
            }

            // Land exactly in diagonal?
            let pos = enemy.position.moved_to(&enemy.direction);
            if !self.board.within_bounds(&pos) || self.board[&pos] != Field::Land {
                enemy.direction = enemy.direction.flipped_x().flipped_y();
            }
        }
    }

    fn move_land_enemies(&mut self) {
        for enemy in self.land_enemies.iter_mut() {
            enemy.position = enemy.position.moved_to(&enemy.direction);
        }
    }

    fn find_collision(&self) -> bool {
        let position = &self.player.position.moved_to(&self.player.direction);
        if self.board.within_bounds(&position) && self.board[&position] == Field::Sand {
            return true;
        }

        for enemy in &self.sea_enemies {
            if enemy.hit_check(|position| {
                *position == self.player.position || self.board[position] == Field::Sand
            }) {
                return true;
            }
        }

        // if land enemy WILL move (or move-vert/move-horiz) into player
        for enemy in &self.land_enemies {
            if enemy.hit_check(|p| *p == self.player.position) {
                return true;
            }
        }

        false
    }

    fn reset(&mut self) {
        self.board.clean();

        self.player = Player {
            position: Position { x: BOARD_WIDTH as i16 / 2, y: 0 },
            direction: Direction { dx: 0, dy: 0 },
        };

        self.land_enemies = vec![
            Enemy {
                position: Position {
                    x: BOARD_WIDTH as i16 / 2,
                    y: BOARD_HEIGHT as i16 - 2,
                },
                direction: random_diagonal(),
            },
        ];
    }
}

impl State for PlayState {
    fn update(&mut self) -> Transition {
        match self.delay {
            Delay::Death(0) => {
                self.delay = Delay::None;
                self.reset();
                return Transition::None;
            },
            Delay::NextLevel(0) => {
                let next_level = Self::new(self.level + 1, self.score, self.lives);
                return Transition::Replace(Box::new(next_level));
            },
            Delay::Death(ref mut counter) | Delay::NextLevel(ref mut counter) => {
                *counter -= 1;
                return Transition::None;
            }
            _ => {},
        }


        self.bounce_sea_enemies();
        self.bounce_land_enemies();

        if self.find_collision() {
            self.lives -= 1;
            if self.lives == 0 {
                return Transition::Push(Box::new(super::GameOverState {}));
            }

            self.delay = Delay::Death(20);
            return Transition::None;
        }

        self.move_player();
        self.move_sea_enemies();
        self.move_land_enemies();

        if self.board.fill_ratio > 0.2 {
            self.delay = Delay::NextLevel(20);
        }

        Transition::None
    }

    fn render(&self, renderer: &mut Renderer) {
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

        let score = format!("Score: {} Xn: {} Full: {:.0}% Time: 90",
                            self.score,
                            self.lives,
                            self.board.fill_ratio * 100.0);
        for (x, c) in score.chars().take(BOARD_WIDTH).enumerate() {
            renderer.put_cell(x as u16, self.board.rows().len() as u16, c);
        }
    }

    fn handle_event(&mut self, event: Event) -> Transition {
        match event {
            Event::Up => {
                self.player.direction = Direction::NORTH;
            },
            Event::Down => {
                self.player.direction = Direction::SOUTH;
            },
            Event::Left => {
                self.player.direction = Direction::WEST;
            },
            Event::Right => {
                self.player.direction = Direction::EAST;
            },
            _ => {},
        }

        Transition::None
    }
}
