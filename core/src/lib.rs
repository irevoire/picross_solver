//! This crate only provide on fuction: `solve`.
//! This function expect the description of the grid as two arrays of arrays,
//! For this grid for example:
//! ```text
//!      | 2 | 1 | 1 | 2 |
//! -----+---+---+---+---+
//!  1 2 | # | X | # | # |
//!  ----+---+---+---+---+
//!  2 1 | # | # | X | # |
//!  ----+---+---+---+---+
//! ```
//! You would call the solve function like this:
//! ```ignore
//! solve( [ [2], [1], [1], [2] ],
//!        [ [1, 2], [2, 1] ]
//! )
//! ```
//! And you will get this in return:
//! ```ignore
//! [
//!     [true, false, true, true],
//!     [true, true, false, true],
//! ]
//! ```

pub use solver::solve;
mod grid;
mod solver;
