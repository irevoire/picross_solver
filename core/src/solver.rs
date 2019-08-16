use crate::grid::*;
use itertools::Itertools;

pub fn solve(width: Vec<Vec<usize>>, height: Vec<Vec<usize>>) -> Vec<Vec<bool>> {
    let mut grid = Grid::new(width.len(), height.len());
    unimplemented!()
    // crate::grid::Grid::from(width.len(), height.len()).solved
}

/// bruteforce one line to find all the cells which will be Full or Empty
pub fn force(line: &mut Vec<Cell>, indications: &Vec<usize>) -> bool {
    let nb_used_cells: usize = indications.iter().sum();
    let nb_unused_cells = line.len() - nb_used_cells;
    // be cautious: the +1 need to be done to avoid going negative
    let nb_moving_cells = nb_unused_cells + 1 - indications.len();

    // is there only one possible solution?
    if nb_moving_cells == 0 {
        let mut index = 0;
        for val in indications {
            for i in indications {
                line[index + i] = Cell::Full;
                index += 1;
            }
            line[index] = Cell::Empty;
            index += 1;
        }
    }

    // we are gonna see which solutions are possible
    let mut nb_solutions = 0;
    let mut force = &vec![0; line.len()];

    let mut groups = create_groups(indications);
    // we need to add the number of moving cells at the end of the groups
    for _ in 0..nb_moving_cells {
        groups.push(vec![Cell::Empty]);
    }

    groups.iter().permutations(groups.len()).for_each(|g| {
        let line: Vec<Cell> = groups.iter().flatten().map(|e| e.clone()).collect();
        // TODO ensure we did not erase some already present things in the
        // line given in parameters
        if is_valid_line(&line, indications) {
            nb_solutions += 1;
            let mut force = line.iter().zip(force).map(|(l, f)| match l {
                Full => *f + 1,
                _ => *f,
            });
        }
    });

    true
}

/// Take a line and the corresponding indications.
/// Return true if the line is a possible solution
fn is_valid_line(line: &[Cell], indications: &[usize]) -> bool {
    let mut curr = &mut line.iter();
    let (last_ind, indications) = indications.split_last().unwrap(); // TODO is this really safe?

    for number in indications {
        // how is this shit working when we are out of the for?
        let curr = &mut curr.skip_while(|&c| c == &Cell::Empty);

        for _ in 0..*number {
            if curr.next() != Some(&Cell::Full) {
                return false;
            }
        }
        if curr.next() != Some(&Cell::Empty) {
            return false;
        }
    }

    // check the last indications, it does not need an Empty cell at the end so
    // we check it out of the loop
    let curr = &mut curr.skip_while(|&c| c == &Cell::Empty);
    for _ in 0..*last_ind {
        if curr.next() != Some(&Cell::Full) {
            return false;
        }
    }

    // Ensure there is only Empty cells left
    curr.all(|c| *c == Cell::Empty)
}

/// Generate groups to allow an easier bruteforce:
/// Fore example for the indication: `[2, 2, 1]`
/// The followings groups will be created:
/// ```text
/// // we can’t put this as rust code because it's private
/// let groups = create_groups(&[2, 2, 1]);
/// assert_eq!(groups,
/// vec![
///     vec![ Full, Full, Empty ],
///     vec![ Full, Full, Empty ],
///     vec![ Full ]
/// ]);
/// ```
/// **It's the caller responsability to ensure there is enough Empty cells on the line**
/// *Return empty array on empty indications*
fn create_groups(indications: &[usize]) -> Vec<Vec<Cell>> {
    let mut groups = Vec::new();

    for i in indications {
        // we are creating group like [F, F, F, E]
        let mut g = Vec::new();
        for _ in 0..*i {
            g.push(Cell::Full);
        }
        g.push(Cell::Empty);
        groups.push(g);
    }
    // The last group don’t need to have it’s last cell Empty
    if let Some(el) = groups.iter_mut().last() {
        el.pop();
    }
    groups
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::Cell::*;

    #[test]
    fn test_create_groups() {
        let groups = create_groups(&[2, 2, 1]);
        assert_eq!(
            groups,
            vec![vec![Full, Full, Empty], vec![Full, Full, Empty], vec![Full]]
        );
        let groups = create_groups(&[12, 1, 1, 1]);
        assert_eq!(
            groups,
            vec![
                vec![Full, Full, Full, Full, Full, Full, Full, Full, Full, Full, Full, Full, Empty],
                vec![Full, Empty],
                vec![Full, Empty],
                vec![Full]
            ]
        );
        let groups = create_groups(&[1]);
        assert_eq!(groups, vec![vec![Full]]);
    }

    #[test]
    fn test_create_groups_empty() {
        let groups = create_groups(&[]);
        assert!(groups.is_empty());
    }

    #[test]
    fn test_is_valid_line() {
        // some kind of true things
        assert_eq!(is_valid_line(&[], &[0]), true);
        assert_eq!(is_valid_line(&[Empty, Empty, Empty], &[0]), true);
        assert_eq!(is_valid_line(&[Full], &[1]), true);
        assert_eq!(is_valid_line(&[Full, Empty], &[1]), true);
        assert_eq!(is_valid_line(&[Empty, Full], &[1]), true);
        assert_eq!(is_valid_line(&[Empty, Full, Empty], &[1]), true);
        assert_eq!(is_valid_line(&[Full, Empty, Full], &[1, 1]), true);
        assert_eq!(
            is_valid_line(
                &[Full, Full, Empty, Empty, Full, Empty, Empty, Full, Full, Empty, Empty],
                &[2, 1, 2]
            ),
            true
        );

        // some kind of false things
        assert_eq!(is_valid_line(&[], &[1]), false);
        assert_eq!(is_valid_line(&[Full, Empty, Full], &[1]), false);
        assert_eq!(is_valid_line(&[Empty, Full, Full], &[1]), false);
        assert_eq!(is_valid_line(&[Full, Empty, Full], &[2]), false);
        assert_eq!(is_valid_line(&[Empty, Empty, Empty], &[1]), false);
        assert_eq!(is_valid_line(&[Full, Full, Empty, Full], &[1, 2]), false);
    }
}
