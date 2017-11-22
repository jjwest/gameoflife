use std::ops::{Index, IndexMut};

#[derive(Copy, Clone)]
pub struct Cell {
    pub alive: bool,
}

pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
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

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.cells.resize(width * height, Cell { alive: false });
        self.width = width;
        self.height = height;
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = Cell;
    fn index(&self, (x, y): (usize, usize)) -> &Cell {
        &self.cells[y * self.width + x]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Cell {
        &mut self.cells[y * self.width + x]
    }
}

impl<'a> IntoIterator for &'a Grid {
    type Item = &'a Cell;
    type IntoIter = ::std::slice::Iter<'a, Cell>;
    fn into_iter(self) -> Self::IntoIter {
        self.cells.iter()
    }
}

impl<'a> IntoIterator for &'a mut Grid {
    type Item = &'a mut Cell;
    type IntoIter = ::std::slice::IterMut<'a, Cell>;
    fn into_iter(self) -> Self::IntoIter {
        self.cells.iter_mut()
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
