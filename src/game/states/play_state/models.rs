use rand::{thread_rng, Rng};
use std::collections::VecDeque;
use std::ops::{Index, IndexMut};
use super::Position;

#[derive(Clone, PartialEq)]
pub enum Field {
    Land,
    Sea,
    DeepSea,
    Sand,
}

pub struct Board {
    fields: Vec<Vec<Field>>,
    width: usize,
    height: usize,
    pub fill_ratio: f64,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        let mut fields = vec![vec![Field::Sea; width]; height];

        for (y, row) in fields.iter_mut().enumerate() {
            for (x, field) in row.iter_mut().enumerate() {
                if x < 2 || x >= width - 2 || y < 2 || y >= height -2 {
                    *field = Field::Land;
                }
            }
        }

        Board {
            fields,
            width,
            height,
            fill_ratio: 0.0
        }
    }

    pub fn rows(&self) -> ::std::slice::Iter<Vec<Field>> {
        self.fields.iter()
    }

    fn random_position(&self) -> Position {
        let mut rng = thread_rng();
        let x = rng.gen_range(0, self.fields[0].len() as i16);
        let y = rng.gen_range(0, self.fields.len() as i16);
        Position { x, y }
    }

    pub fn random_position_of_type(&self, field_type: Field) -> Position {
        let mut pos = self.random_position();
        while self.fields[pos.y as usize][pos.x as usize] != field_type {
            pos = self.random_position();
        }
        pos
    }

    pub fn fill(&mut self, enemy_positions: &[&Position]) {
        fn flood_fill(fields: &mut Vec<Vec<Field>>, position: (i16, i16)) {
            let mut q = VecDeque::new();
            q.push_back(position);
            while !q.is_empty() {
                let (x, y) = q.pop_front().unwrap();
                if fields[y as usize][x as usize] == Field::Sea {
                    fields[y as usize][x as usize] = Field::DeepSea;
                    q.push_back((x, y - 1));
                    q.push_back((x, y + 1));
                    q.push_back((x - 1, y));
                    q.push_back((x + 1, y));
                }
            }
        }

        for pos in enemy_positions {
            flood_fill(&mut self.fields, (pos.x, pos.y));
        }

        let mut remaining_sea_fields = 0;
        for row in self.fields.iter_mut() {
            for field in row.iter_mut() {
                if *field == Field::DeepSea {
                    *field = Field::Sea;
                    remaining_sea_fields += 1;
                } else if *field == Field::Sea || *field == Field::Sand {
                    *field = Field::Land;
                }
            }
        }

        let total_sea_fields = (self.width - 4) * (self.height - 4);
        self.fill_ratio = 1.0 - remaining_sea_fields as f64 / total_sea_fields as f64;
    }

    pub fn within_bounds(&self, position: &Position) -> bool {
        position.x >= 0 && position.x < self.fields[0].len() as i16 &&
            position.y >= 0 && position.y < self.fields.len() as i16
    }

    pub fn clean(&mut self) {
        for row in self.fields.iter_mut() {
            for field in row.iter_mut() {
                if *field == Field::Sand {
                    *field = Field::Sea;
                }
            }
        }
    }
}

impl<'a> Index<&'a Position> for Board {
    type Output = Field;

    fn index(&self, position: &Position) -> &Field {
        &self.fields[position.y as usize][position.x as usize]
    }
}

impl<'a> IndexMut<&'a Position>  for Board {
    fn index_mut(&mut self, position: &Position) -> &mut Field {
        &mut self.fields[position.y as usize][position.x as usize]
    }
}
