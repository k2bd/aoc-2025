use std::{collections::HashSet, ops::Add};

use pyo3::{ffi::traverseproc, prelude::*};
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pyfunction, gen_stub_pymethods};

#[pymodule(module = "aoc_2026.rs.day09")]
pub fn day9(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(largest_carpet_area, m)?)?;
    m.add_class::<FactoryFloor>()?;

    Ok(())
}

#[gen_stub_pyclass]
#[pyclass(module = "aoc_2025.rs.day09")]
#[derive(Hash, PartialEq, Eq, PartialOrd, Clone, Copy, Debug)]
struct Coordinate {
    #[pyo3(get)]
    x: isize,
    #[pyo3(get)]
    y: isize,
}

impl Add<Coordinate> for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Coordinate) -> Self::Output {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Coordinate {
    fn neighbors(&self) -> HashSet<Coordinate> {
        HashSet::from([
            *self + Coordinate { x: -1, y: 0 },
            *self + Coordinate { x: 1, y: 0 },
            *self + Coordinate { x: 0, y: -1 },
            *self + Coordinate { x: 0, y: 1 },
        ])
    }
}

fn area(p1: Coordinate, p2: Coordinate) -> usize {
    (((p1.x - p2.x).abs() + 1) * ((p1.y - p2.y).abs() + 1)) as usize
}

#[gen_stub_pyfunction(module = "aoc_2025.rs.day09")]
#[pyfunction]
/// Largest carpet area for part 1
fn largest_carpet_area(input: &str) -> usize {
    let points = input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            Coordinate {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    points
        .iter()
        .flat_map(|p1| {
            points.iter().filter_map(move |p2| {
                if p1 >= p2 {
                    None
                } else {
                    Some((p1.to_owned(), p2.to_owned()))
                }
            })
        })
        .map(|(p1, p2)| area(p1, p2))
        .max()
        .unwrap()
}

#[gen_stub_pyclass]
#[pyclass(module = "aoc_2025.rs.day09")]
struct FactoryFloor {
    red_tiles: HashSet<Coordinate>,
    green_area: HashSet<Coordinate>,
}

/// A horrible way to get the green area. Trace the bounding lines of the
/// shape, flood-fill outside the shape, then invert
fn get_green_area(red_path: &Vec<Coordinate>) -> HashSet<Coordinate> {
    let mut path_pairs = red_path.clone();
    path_pairs.rotate_left(1);

    // Path around the shape
    let mut path_elements = HashSet::new();
    for (from, to) in red_path.clone().into_iter().zip(path_pairs) {
        if from.x == to.x {
            let start = from.y.min(to.y);
            let end = from.y.max(to.y);

            (start..=end).for_each(|y| {
                path_elements.insert(Coordinate { x: from.x, y });
            });
        } else {
            let start = from.x.min(to.x);
            let end = from.x.max(to.x);

            (start..=end).for_each(|x| {
                path_elements.insert(Coordinate { x, y: from.y });
            });
        }
    }

    // Now flood fill the inside of the shape
    let min_coord = Coordinate {
        x: red_path.iter().map(|c| c.x).min().unwrap() - 1,
        y: red_path.iter().map(|c| c.y).min().unwrap() - 1,
    };
    let max_coord = Coordinate {
        x: red_path.iter().map(|c| c.x).max().unwrap() + 1,
        y: red_path.iter().map(|c| c.y).max().unwrap() + 1,
    };

    // Find an inner point by finding a row without any points and whose first
    // two boundaries that have at least one point between them.
    // N.B. ASSUMES there will be some inner point of this kind, though that's
    // not necessarily the case in general
    let point_inside_shape = (min_coord.y..max_coord.y)
        .filter(|&y| !red_path.iter().any(|&c| c.y == y))
        .find_map(|y| {
            let mut elements = path_elements
                .iter()
                .cloned()
                .filter(move |&c| c.y == y)
                .collect::<Vec<_>>();
            elements.sort_by_key(|c| c.x);
            let mut elements_iter = elements.into_iter();

            if let Some(target) = elements_iter.next() {
                if let Some(next) = elements_iter.next() {
                    if next.x - target.x > 1 {
                        return Some(target + Coordinate { x: 1, y: 0 });
                    }
                }
            }

            None
        })
        .unwrap();

    let mut q: Vec<Coordinate> = Vec::from([point_inside_shape]);
    let mut inside_shape: HashSet<Coordinate> = HashSet::new();

    let tmp_perimeter_points = path_elements.len();
    let approx_area = ((tmp_perimeter_points as f64) / 4.0).powf(2.0);

    loop {
        if q.is_empty() {
            break;
        }

        let n = q.pop().unwrap();

        if !path_elements.contains(&n) {
            inside_shape.insert(n);
            println!(
                "{:?} - ~{:?}%",
                n,
                100.0 * (inside_shape.len() as f64 / approx_area)
            );
            for neighbor in n.neighbors() {
                if !inside_shape.contains(&neighbor) {
                    q.push(neighbor);
                }
            }
        }
    }
    inside_shape.extend(path_elements);

    inside_shape
}

fn perimeter_points(c1: Coordinate, c2: Coordinate) -> HashSet<Coordinate> {
    let mut result = HashSet::new();
    let min_x = c1.x.min(c2.x);
    let min_y = c1.y.min(c2.y);
    let max_x = c1.x.max(c2.x);
    let max_y = c1.y.max(c2.y);

    (min_x..=max_x)
        .flat_map(|x| [Coordinate { x, y: min_y }, Coordinate { x, y: max_y }])
        .for_each(|coord| {
            result.insert(coord);
        });
    (min_y..=max_y)
        .flat_map(|y| [Coordinate { x: min_x, y }, Coordinate { x: max_x, y }])
        .for_each(|coord| {
            result.insert(coord);
        });

    result
}

impl From<&str> for FactoryFloor {
    fn from(value: &str) -> Self {
        let red_path = value
            .lines()
            .map(|l| {
                let (x, y) = l.split_once(",").unwrap();
                Coordinate {
                    x: x.parse().unwrap(),
                    y: y.parse().unwrap(),
                }
            })
            .collect::<Vec<_>>();

        let green_area = get_green_area(&red_path);

        Self {
            red_tiles: red_path.into_iter().collect(),
            green_area,
        }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl FactoryFloor {
    #[new]
    fn new(input: &str) -> Self {
        Self::from(input)
    }

    fn largest_carpet_area(&self) -> usize {
        self.red_tiles
            .iter()
            .flat_map(|p1| {
                self.red_tiles.iter().filter_map(move |p2| {
                    if p1 >= p2 {
                        None
                    } else {
                        Some((p1.to_owned(), p2.to_owned()))
                    }
                })
            })
            .filter(|&(c1, c2)| {
                perimeter_points(c1, c2)
                    .iter()
                    .all(|p| self.green_area.contains(p))
            })
            .map(|(c1, c2)| area(c1, c2))
            .max()
            .unwrap()
    }
}
