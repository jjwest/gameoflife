#[derive(Copy, Clone)]
pub struct Cell {
    pub alive: bool,
}

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Grid {
            width,
            height,
            cells: vec![Cell { alive: false }; width * height],
        }
    }

    pub fn next_generation(&mut self) {
        let mut next_gen = self.cells.clone();

        for (i, cell) in self.cells.iter().enumerate() {
            let neighbours = self.count_alive_neighbours(i);

            if cell.alive {
                if neighbours < 2 || neighbours > 3 {
                    next_gen[i].alive = false;
                }
            } else if neighbours == 3 {
                next_gen[i].alive = true;
            }
        }

        self.cells = next_gen;
    }

    fn count_alive_neighbours(&self, origin: usize) -> usize {
        let mut neighbours = [None; 8];

        // Check left side neighbours if we are not in the first column
        if origin % self.width != 0 {
            neighbours[0] = self.cells.get(origin.wrapping_sub(self.width - 1));
            neighbours[1] = self.cells.get(origin.wrapping_sub(1));
            neighbours[2] = self.cells.get(origin + self.width - 1);
        }

        // Check top and bottom neighbours
        neighbours[3] = self.cells.get(origin.wrapping_sub(self.width));
        neighbours[4] = self.cells.get(origin + self.width);

        // Check right side neighbours if we are not in the last column
        if origin % self.width != self.width - 1 {
            neighbours[5] = self.cells.get(origin.wrapping_sub(self.width + 1));
            neighbours[6] = self.cells.get(origin + self.width + 1);
            neighbours[7] = self.cells.get(origin + 1);
        }

        neighbours
            .iter()
            .filter_map(|&n| n)
            .filter(|&n| n.alive)
            .count()
    }

    pub fn get(&self, x: usize, y: usize) -> Cell {
        self.cells[y * self.width + x]
    }

    pub fn toggle_cell_alive(&mut self, x: usize, y: usize) {
        let cell = &mut self.cells[y * self.width + x];
        cell.alive = !cell.alive;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_generation() {
        let alive = Cell { alive: true };
        let dead = Cell { alive: false };

        let grid = Grid {
            width: 5,
            height: 5,
            cells: vec![
                alive,
                dead,
                dead,
                alive,
                alive,
                alive,
                alive,
                dead,
                dead,
                dead,
                dead,
                alive,
                dead,
                dead,
                dead,
                dead,
                dead,
                dead,
                dead,
                dead,
                dead,
                dead,
                dead,
                dead,
                dead,
            ],
        };

        assert_eq!(3, grid.count_alive_neighbours(5));
    }
}
