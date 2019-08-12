//! This crate only provide on fuction: `solve`.
//! This function expect the description of the grid as two arrays of arrays,
//! For this grid for example:
//! ```
//!      | 2 | 1 | 1 | 2 |
//! -----+---+---+---+---+
//!  1 2 | # | X | # | # |
//!  ----+---+---+---+---+
//!  2 1 | # | # | X | # |
//!  ----+---+---+---+---+
//! ```
//! You would call the solve function like this:
//! ```rust
//! solve( [ [2], [1], [1], [2] ],
//!        [ [1, 2], [2, 1] ]
//! )
//! ```
//! And you will get this in return:
//! ```rust
//! [
//!     [true, false, true, true],
//!     [true, true, false, true],
//! ]
//! ```

pub use solver::solve;
mod grid;
mod solver;
