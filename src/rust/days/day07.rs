use std::{collections::HashSet, ops::Add};

use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

#[pymodule(module = "aoc_2026.rs.day07")]
pub fn day7(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Coordinate>()?;
    m.add_class::<TachyonManifold>()?;

    Ok(())
}

#[gen_stub_pyclass]
#[pyclass(module = "aoc_2025.rs.day07")]
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Coordinate {
    #[pyo3(get)]
    x: isize,
    #[pyo3(get)]
    y: isize,
}

impl Add<Coordinate> for Coordinate {
    type Output = Self;

    fn add(self, rhs: Coordinate) -> Self::Output {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[gen_stub_pyclass]
#[pyclass(module = "aoc_2025.rs.day07")]
#[derive(PartialEq, Debug)]
struct TachyonManifold {
    /// Coordinates of the beam fronts
    #[pyo3(get)]
    beam_fronts: HashSet<Coordinate>,

    /// Coordinates of splitters
    #[pyo3(get)]
    splitter_locations: HashSet<Coordinate>,
}

impl From<&str> for TachyonManifold {
    fn from(value: &str) -> Self {
        let (beam_fronts, splitter_locations) = value
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars().enumerate().map(move |(col, c)| {
                    (
                        Coordinate {
                            x: col as isize,
                            y: row as isize,
                        },
                        c,
                    )
                })
            })
            .fold(
                (HashSet::<Coordinate>::new(), HashSet::<Coordinate>::new()),
                |(mut beams, mut splitters), (coord, value)| {
                    if value == 'S' {
                        beams.insert(coord);
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
    fn propagate(&mut self) -> Option<usize> {
        if self.beam_fronts.iter().map(|front| front.y).min()
            >= self.splitter_locations.iter().map(|loc| loc.y).max()
        {
            return None;
        }

        let unpropagated_fronts = self.beam_fronts.clone();
        self.beam_fronts = HashSet::new();

        let mut result = 0;

        for beam_front in unpropagated_fronts {
            let new_coord = beam_front + Coordinate { x: 0, y: 1 };
            if self.splitter_locations.contains(&new_coord) {
                self.beam_fronts
                    .insert(new_coord + Coordinate { x: -1, y: 0 });
                self.beam_fronts
                    .insert(new_coord + Coordinate { x: 1, y: 0 });
                result += 1;
            } else {
                self.beam_fronts.insert(new_coord);
            }
        }

        Some(result)
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
                beam_fronts: HashSet::from([Coordinate { x: 7, y: 0 }]),
                splitter_locations: HashSet::from([Coordinate { x: 7, y: 2 }]),
            }
        );
    }

    #[rstest]
    fn test_propagate() {
        let mut manifold = TachyonManifold {
            beam_fronts: HashSet::from([Coordinate { x: 7, y: 0 }]),
            splitter_locations: HashSet::from([Coordinate { x: 7, y: 2 }]),
        };

        assert_eq!(manifold.propagate(), Some(0));

        assert_eq!(
            manifold.beam_fronts,
            HashSet::from([Coordinate { x: 7, y: 1 }]),
        );

        assert_eq!(manifold.propagate(), Some(1));

        assert_eq!(
            manifold.beam_fronts,
            HashSet::from([Coordinate { x: 6, y: 2 }, Coordinate { x: 8, y: 2 }]),
        );

        assert_eq!(manifold.propagate(), None);

        assert_eq!(
            manifold.beam_fronts,
            HashSet::from([Coordinate { x: 6, y: 2 }, Coordinate { x: 8, y: 2 }]),
        );
    }
}
