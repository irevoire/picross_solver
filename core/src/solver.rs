pub fn solve(width: Vec<Vec<u32>>, height: Vec<Vec<u32>>) -> Vec<Vec<bool>> {
    crate::grid::Grid::from(width, height).solved
}
