use rand::Rng;
use std::io::{stdout, Write};

pub struct Life {
    cells: Vec<Vec<bool>>,
    temp: Vec<Vec<bool>>,
}

impl Life {
    pub fn new(size: (u16, u16)) -> Life {
        let mut life = Life {
            cells: Vec::with_capacity(size.1 as usize),
            temp: Vec::with_capacity(size.1 as usize),
        };

        for _y in 0..size.1 {
            let mut row: Vec<bool> = Vec::with_capacity(size.0 as usize);
            let mut temp_row: Vec<bool> = Vec::with_capacity(size.0 as usize);

            for _x in 0..size.0 {
                row.push(false);
                temp_row.push(false);
            }

            life.cells.push(row);
            life.temp.push(temp_row);
        }

        life
    }

    pub fn randomize(&mut self, chance: u8) {
        for y in 0..self.cells.len() {
            for x in 0..self.cells[y].len() {
                self.cells[y][x] = rand::thread_rng().gen_range(u8::MIN, u8::MAX) < chance;
            }
        }
    }

    pub fn update(&mut self) {
        for y in 0..self.cells.len() {
            for x in 0..self.cells[y].len() {
                let mut total: u8 = 0;

                let left = x == 0;
                let right = x == self.cells[y].len() - 1;
                let top = y == 0;
                let bottom = y == self.cells.len() - 1;

                if !left && self.cells[y][x - 1] {
                    total += 1;
                }
                if !right && self.cells[y][x + 1] {
                    total += 1;
                }
                if !top && self.cells[y - 1][x] {
                    total += 1;
                }
                if !bottom && self.cells[y + 1][x] {
                    total += 1;
                }

                if !left && !top && self.cells[y - 1][x - 1] {
                    total += 1;
                }
                if !left && !bottom && self.cells[y + 1][x - 1] {
                    total += 1;
                }
                if !right && !top && self.cells[y - 1][x + 1] {
                    total += 1;
                }
                if !right && !bottom && self.cells[y + 1][x + 1] {
                    total += 1;
                }

                self.temp[y][x] = !(total < 2 || total > 3 || (!self.cells[y][x] && total != 3));
            }
        }

        for y in 0..self.cells.len() {
            for x in 0..self.cells[y].len() {
                self.cells[y][x] = self.temp[y][x];
            }
        }
    }

    pub fn print(&self) {
        print!("{}[2J", 27 as char);
        print!("{}[H", 27 as char);

        if let Err(_err) = stdout().flush() {}

        let res: Vec<String> = self
            .cells
            .clone()
            .into_iter()
            .map(|y| {
                let res: Vec<&str> = y
                    .clone()
                    .into_iter()
                    .map(|x| if x { "#" } else { " " })
                    .collect();

                res.join(" ")
            })
            .collect();

        print!("{}\n", res.join("\n"));
    }
}
