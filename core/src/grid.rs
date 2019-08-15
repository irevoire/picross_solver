/// Representation of a Cell
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Cell {
    Full,
    Empty,
    Unknown,
}

use Cell::*;

/// Store all the informations about the grid state and the solved grid
/// A finished grid should only contain Empty and Full cells but no Unknown.
/// When initializing a grid, a vector of the size width * height will be created
/// and filled with unknown Cells.
pub struct Grid {
    grid: Vec<Vec<Cell>>,
}

impl Grid {
    /// generate an empty `Grid`.
    pub fn new(width: usize, height: usize) -> Grid {
        Grid {
            grid: vec![vec![Unknown; width]; height],
        }
    }

    /// return the (width, height) combinaison as a tuple
    pub fn dim(&self) -> (usize, usize) {
        (self.grid[0].len(), self.grid.len())
    }

    /// return true if there is no Unknown cell left
    pub fn solved(&self) -> bool {
        self.grid.iter().flatten().all(|c| match c {
            Unknown => false,
            _ => true,
        })
    }

    /// provide a copy of a line
    pub fn line(&mut self, index: usize) -> Vec<Cell> {
        self.grid[index].iter().map(Cell::clone).collect()
    }

    /// provide a copy of a column
    pub fn col(&mut self, index: usize) -> Vec<Cell> {
        self.grid.iter().map(|c| c[index].clone()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len_grid() {
        let grid = Grid::new(2, 2);
        assert_eq!(grid.dim(), (2, 2));
    }

    #[test]
    fn solved_grid() {
        let mut grid = Grid::new(2, 2);
        grid.grid[0][0] = Full;
        assert_eq!(grid.solved(), false);
        grid.grid[0][1] = Empty;
        assert_eq!(grid.solved(), false);
        grid.grid[1][0] = Empty;
        assert_eq!(grid.solved(), false);
        grid.grid[1][1] = Full;
        assert_eq!(grid.solved(), true);
    }

    #[test]
    fn line_grid() {
        let mut grid = Grid::new(2, 2);
        grid.grid[0][0] = Full;
        grid.grid[0][1] = Empty;
        grid.grid[1][0] = Empty;
        grid.grid[1][1] = Full;
        assert_eq!(grid.line(0), vec![Full, Empty]);
        assert_eq!(grid.line(1), vec![Empty, Full]);
    }

    #[test]
    fn column_grid() {
        let mut grid = Grid::new(2, 2);
        grid.grid[0][0] = Full;
        grid.grid[0][1] = Empty;
        grid.grid[1][0] = Empty;
        grid.grid[1][1] = Full;
        assert_eq!(grid.col(0), vec![Full, Empty]);
        assert_eq!(grid.col(1), vec![Empty, Full]);
    }
}
