use std::collections::HashSet;

use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

#[pymodule(module = "aoc_2025.rs.day04")]
pub fn day4(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<StorageRoom>()?;

    Ok(())
}

#[gen_stub_pyclass]
#[pyclass(module = "aoc_2025.rs.day04")]
struct StorageRoom {
    rolls: HashSet<(isize, isize)>,
}

impl From<&str> for StorageRoom {
    fn from(value: &str) -> Self {
        Self {
            rolls: value
                .lines()
                .enumerate()
                .flat_map(|(row_index, row)| {
                    row.char_indices()
                        .filter(|&(_, char)| char == '@')
                        .map(move |(col_index, _)| (row_index as isize, col_index as isize))
                })
                .collect(),
        }
    }
}

impl StorageRoom {
    fn neighbour_count(&self, (row, col): (isize, isize)) -> usize {
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .map(|(dr, dc)| (row + dr, col + dc))
        .iter()
        .filter(|&n| self.rolls.contains(n))
        .count()
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl StorageRoom {
    #[new]
    fn new(rolls: &str) -> Self {
        Self::from(rolls)
    }

    /// Get the (row, col) coordinates of rolls that have fewer than 4 adjacent
    /// rolls
    fn accessible_rolls(&self) -> HashSet<(isize, isize)> {
        self.rolls
            .iter()
            .filter(|&&pos| self.neighbour_count(pos) < 4)
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    fn test_create_storage_room() {
        let example_input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@";
        let room = StorageRoom::from(example_input);
        assert_eq!(
            room.rolls,
            HashSet::from([
                (0, 2),
                (0, 3),
                (0, 5),
                (0, 6),
                (0, 7),
                (0, 8),
                (1, 0),
                (1, 1),
                (1, 2),
                (1, 4),
                (1, 6),
                (1, 8),
                (1, 9),
                (2, 0),
                (2, 1),
                (2, 2),
                (2, 3),
                (2, 4),
                (2, 6),
                (2, 8),
                (2, 9)
            ])
        );
    }

    #[rstest]
    #[case((0, 5), 3)]
    #[case((0, 6), 3)]
    fn test_neighbour_count(#[case] pos: (isize, isize), #[case] expected: usize) {
        let example_input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@@@@@@@@@@\n@@@@@@@@@@";
        let room = StorageRoom::from(example_input);
        assert_eq!(room.neighbour_count(pos), expected);
    }

    #[rstest]
    fn test_accessible_rolls() {
        let example_input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@@@@@@@@@@\n@@@@@@@@@@";
        let room = StorageRoom::from(example_input);
        assert_eq!(
            room.accessible_rolls(),
            HashSet::from([
                (0, 2),
                (0, 3),
                (0, 5),
                (0, 6),
                (0, 8),
                (1, 0),
                (4, 0),
                (4, 9)
            ])
        );
    }
}
