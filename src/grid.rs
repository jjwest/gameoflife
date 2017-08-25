#[derive(Copy, Clone, Debug)]
pub struct Cell {
    pub alive: bool,
}

#[derive(Clone, Debug)]
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
            cells: {
                let mut cells = vec![Cell { alive: false }; width * height];
                cells[width / 2].alive = true;
                cells[width / 2 - 1].alive = true;
                cells[width / 2 - 2].alive = true;
                cells
            },
        }
    }

    pub fn next_generation(&mut self) {
        let mut next_gen = self.cells.clone();

        for i in 0..self.cells.len() {
            let neighbours = self.count_alive_neighbours(i);

            if self.cells[i].alive {
                if neighbours < 2 || neighbours > 3 {
                    next_gen[i].alive = false;
                }
            } else {
                if neighbours == 3 {
                    next_gen[i].alive = true;
                }
            }
        }

        self.cells = next_gen;
    }

    fn count_alive_neighbours(&self, cell: usize) -> usize {
        let mut neighbours = [None; 8];

        if cell % self.width != 0 {
            neighbours[0] = self.cells.get(cell.wrapping_sub(self.width - 1));
            neighbours[1] = self.cells.get(cell.wrapping_sub(1));
            neighbours[2] = self.cells.get(cell + self.width - 1);
        }

        neighbours[3] = self.cells.get(cell.wrapping_sub(self.width));
        neighbours[4] = self.cells.get(cell + self.width);

        if cell % self.width != self.width - 1 {
            neighbours[5] = self.cells.get(cell.wrapping_sub(self.width + 1));
            neighbours[6] = self.cells.get(cell + self.width + 1);
            neighbours[7] = self.cells.get(cell + 1);
        }

        neighbours
            .iter()
            .filter_map(|&n| n)
            .filter(|&n| n.alive)
            .count()
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
