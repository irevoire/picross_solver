/// Store all the informations about the grid state and the solved grid
pub struct Grid {
    width: Vec<Vec<u32>>,
    height: Vec<Vec<u32>>,

    pub solved: Vec<Vec<bool>>,
}

impl Grid {
    /// generate an empty `Grid` from the indication specified
    pub fn from(width: Vec<Vec<u32>>, height: Vec<Vec<u32>>) -> Grid {
        Grid {
            solved: vec![vec![false; width.len()]; height.len()],

            width,
            height,
        }
    }
}
