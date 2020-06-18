use std::convert::TryInto;

pub struct World {
    pub width: u32,
    pub height: u32,
    cells: Vec<bool>,
}

impl World {
    // TODO: implement iter for World
    pub fn new(w: u32, h: u32) -> World {
        let mut vec = Vec::with_capacity(1.try_into().unwrap());
        vec.resize((w * h) as usize, false);

        World {
            width: w,
            height: h,
            cells: vec,
        }
    }

    pub fn tick(&mut self) {
        let mut next_cells = self.cells.clone();
        let mut i: u32;
        let mut j: u32;

        for (idx, cell) in self.cells.iter().enumerate() {
            i = (idx as u32) % self.width;
            j = (idx as u32) / self.width;
            if *cell {
                // Cell is alive
                let neighbours = self.count_neighbours_at(i, j);
                match neighbours {
                    // underpopulation
                    0 | 1 => next_cells[idx] = false,
                    // just right
                    2 | 3 => next_cells[idx] = true,
                    // overpopulation
                    _ => next_cells[idx] = false,
                }
            } else {
                // Cell is dead
                next_cells[idx] = self.count_neighbours_at(i, j) == 3
            }
        }

        self.cells = next_cells;
    }

    pub fn print(&self) {
        for (idx, cell) in self.cells.iter().enumerate() {
            if (idx + 1) as u32 % self.width == 0 {
                println!("{}", if *cell { "#" } else { "_" })
            } else {
                print!("{}", if *cell { "#" } else { "_" })
            }
        }
    }

    pub fn get_cell_at(&self, i: u32, j: u32) -> bool {
        self.cells[(self.width * j + i) as usize]
    }

    pub fn set_cell_at(&mut self, i: u32, j: u32, value: bool) {
        self.cells[(self.width * j + i) as usize] = value;
    }

    fn count_neighbours_at(&self, i: u32, j: u32) -> u8 {
        // handle edge cases
        if i == 0 || j == 0 {
            return 0;
        }
        if i >= self.width - 2 || j >= self.height - 2 {
            return 0;
        }
        [
            self.get_cell_at(i - 1, j - 1),
            self.get_cell_at(i, j - 1),
            self.get_cell_at(i - 1, j),
            self.get_cell_at(i, j + 1),
            self.get_cell_at(i + 1, j + 1),
            self.get_cell_at(i + 1, j),
            self.get_cell_at(i + 1, j - 1),
            self.get_cell_at(i - 1, j + 1),
        ]
        .iter()
        .filter(|a| **a)
        .count() as u8
    }
}
