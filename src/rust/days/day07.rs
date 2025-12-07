use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

#[pymodule(module = "aoc_2026.rs.day07")]
pub fn day7(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<TachyonManifold>()?;

    Ok(())
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Coordinate(isize, isize);

impl Add<Coordinate> for Coordinate {
    type Output = Self;

    fn add(self, rhs: Coordinate) -> Self::Output {
        Coordinate(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[gen_stub_pyclass]
#[pyclass(module = "aoc_2025.rs.day07")]
#[derive(PartialEq, Debug)]
struct TachyonManifold {
    /// Coordinates of the beam fronts to number of beams at that position
    beam_fronts: HashMap<Coordinate, usize>,

    /// Coordinates of splitters
    splitter_locations: HashSet<Coordinate>,
}

impl From<&str> for TachyonManifold {
    fn from(value: &str) -> Self {
        let (beam_fronts, splitter_locations) = value
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(col, c)| (Coordinate(col as isize, row as isize), c))
            })
            .fold(
                (
                    HashMap::<Coordinate, usize>::new(),
                    HashSet::<Coordinate>::new(),
                ),
                |(mut beams, mut splitters), (coord, value)| {
                    if value == 'S' {
                        let entry = beams.entry(coord).or_insert(0);
                        *entry += 1;
                    } else if value == '^' {
                        splitters.insert(coord);
                    }
                    (beams, splitters)
                },
            );

        Self {
            beam_fronts,
            splitter_locations,
        }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl TachyonManifold {
    #[new]
    fn new(value: &str) -> Self {
        Self::from(value)
    }

    /// Propagate the system one step, returning the numbers of splits that occurred.
    /// Returns None if all the fronts have cleared all the splitters
    fn propagate(&mut self, combine_beams: bool) -> Option<usize> {
        if self.beam_fronts.keys().map(|front| front.1).min()
            >= self.splitter_locations.iter().map(|loc| loc.1).max()
        {
            return None;
        }

        let unpropagated_fronts = self.beam_fronts.clone();
        self.beam_fronts = HashMap::new();

        let mut total_splits = 0;

        for (beam_front, count) in unpropagated_fronts {
            let new_coord = beam_front + Coordinate(0, 1);
            if self.splitter_locations.contains(&new_coord) {
                let left_entry = self
                    .beam_fronts
                    .entry(new_coord + Coordinate(-1, 0))
                    .or_insert(0);
                *left_entry += count;
                let right_entry = self
                    .beam_fronts
                    .entry(new_coord + Coordinate(1, 0))
                    .or_insert(0);
                *right_entry += count;

                total_splits += count;
            } else {
                let entry = self.beam_fronts.entry(new_coord).or_insert(0);
                *entry += count;
            }
        }

        if combine_beams {
            self.beam_fronts
                .iter_mut()
                .for_each(|(_, value)| *value = 1);
        }

        Some(total_splits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    fn test_from_str() {
        let test_input = ".......S.......\n...............\n.......^.......";
        assert_eq!(
            TachyonManifold::from(test_input),
            TachyonManifold {
                beam_fronts: HashMap::from([(Coordinate(7, 0), 1)]),
                splitter_locations: HashSet::from([Coordinate(7, 2)]),
            }
        );
    }

    #[rstest]
    fn test_propagate() {
        let mut manifold = TachyonManifold {
            beam_fronts: HashMap::from([(Coordinate(7, 0), 1)]),
            splitter_locations: HashSet::from([Coordinate(7, 2)]),
        };

        assert_eq!(manifold.propagate(true), Some(0));

        assert_eq!(manifold.beam_fronts, HashMap::from([(Coordinate(7, 1), 1)]),);

        assert_eq!(manifold.propagate(true), Some(1));

        assert_eq!(
            manifold.beam_fronts,
            HashMap::from([(Coordinate(6, 2), 1), (Coordinate(8, 2), 1)]),
        );

        assert_eq!(manifold.propagate(true), None);

        assert_eq!(
            manifold.beam_fronts,
            HashMap::from([(Coordinate(6, 2), 1), (Coordinate(8, 2), 1)]),
        );
    }
}
