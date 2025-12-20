use std::{collections::HashSet, ops::Add};

use pyo3::prelude::*;
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

    shape_outline: HashSet<Coordinate>,
    vertical_walls: HashSet<Coordinate>,
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

/// Get the coordinates of the shape's outline
fn get_shape_outline(red_path: &[Coordinate]) -> HashSet<Coordinate> {
    let mut path_pairs = red_path.to_owned();
    path_pairs.rotate_left(1);
    // Path around the shape
    red_path
        .iter()
        .zip(path_pairs)
        .flat_map(|(from, to)| {
            (from.y.min(to.y)..=from.y.max(to.y)).flat_map(move |y| {
                (from.x.min(to.x)..=from.x.max(to.x)).map(move |x| Coordinate { x, y })
            })
        })
        .collect()
}

/// Get the coordinates of all the vertical walls
fn get_vertical_walls(red_path: &[Coordinate]) -> HashSet<Coordinate> {
    let mut path_pairs = red_path.to_vec();
    path_pairs.rotate_left(1);
    // Path around the shape
    red_path
        .iter()
        .zip(path_pairs)
        .filter(|(from, to)| from.x == to.x)
        .flat_map(|(from, to)| {
            (from.y.min(to.y)..from.y.max(to.y))
                .map(|y| Coordinate { x: from.x, y })
                .collect::<Vec<_>>()
        })
        .collect()
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

        let shape_outline = get_shape_outline(&red_path);
        let vertical_walls = get_vertical_walls(&red_path);

        Self {
            red_tiles: red_path.into_iter().collect(),
            shape_outline,
            vertical_walls,
        }
    }
}

impl FactoryFloor {
    /// Returns if the point is inside the polygon
    fn point_inside(&self, point: Coordinate) -> bool {
        self.shape_outline.contains(&point)
            || self
                .vertical_walls
                .iter()
                .filter(|p| p.y == point.y && p.x >= point.x)
                .count()
                % 2
                == 1
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
        let mut tile_areas = self
            .red_tiles
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
            .map(|(c1, c2)| (c1, c2, area(c1, c2)))
            .collect::<Vec<_>>();
        tile_areas.sort_by_key(|&(_, _, area)| -(area as isize));

        tile_areas
            .into_iter()
            .find(|&(c1, c2, _)| {
                println!("{:?} - {:?}", c1, c2);
                perimeter_points(c1, c2)
                    .iter()
                    .all(|&p| self.point_inside(p))
            })
            .map(|(_, _, area)| area)
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    fn test_shape_outline() {
        // Shape reminder:
        // ..............
        // .......#XXX#..
        // .......X...X..
        // ..#XXXX#...X..
        // ..X........X..
        // ..#XXXXXX#.X..
        // .........X.X..
        // .........#X#..
        // ..............

        let input = "7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3";
        let floor = FactoryFloor::from(input);

        assert_eq!(
            floor.shape_outline,
            HashSet::from([
                // Vertical
                Coordinate { x: 7, y: 1 },
                Coordinate { x: 7, y: 2 },
                Coordinate { x: 7, y: 3 },
                Coordinate { x: 11, y: 1 },
                Coordinate { x: 11, y: 2 },
                Coordinate { x: 11, y: 3 },
                Coordinate { x: 11, y: 4 },
                Coordinate { x: 11, y: 5 },
                Coordinate { x: 11, y: 6 },
                Coordinate { x: 11, y: 7 },
                Coordinate { x: 9, y: 7 },
                Coordinate { x: 9, y: 6 },
                Coordinate { x: 9, y: 5 },
                Coordinate { x: 2, y: 3 },
                Coordinate { x: 2, y: 4 },
                Coordinate { x: 2, y: 5 },
                // Horizontal
                Coordinate { x: 8, y: 1 },
                Coordinate { x: 9, y: 1 },
                Coordinate { x: 10, y: 1 },
                Coordinate { x: 3, y: 3 },
                Coordinate { x: 4, y: 3 },
                Coordinate { x: 5, y: 3 },
                Coordinate { x: 6, y: 3 },
                Coordinate { x: 3, y: 5 },
                Coordinate { x: 4, y: 5 },
                Coordinate { x: 5, y: 5 },
                Coordinate { x: 6, y: 5 },
                Coordinate { x: 7, y: 5 },
                Coordinate { x: 8, y: 5 },
                Coordinate { x: 5, y: 5 },
                Coordinate { x: 10, y: 7 },
            ])
        );
    }

    #[rstest]
    fn test_vertical_walls() {
        // Shape reminder:
        // ..............
        // .......#XXX#..
        // .......X...X..
        // ..#XXXX#...X..
        // ..X........X..
        // ..#XXXXXX#.X..
        // .........X.X..
        // .........#X#..
        // ..............

        // Vertical walls:
        // ..............
        // .......X...X..
        // .......X...X..
        // ..X....-...X..
        // ..X........X..
        // ..-......X.X..
        // .........X.X..
        // .........-.-..
        // ..............

        let input = "7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3";
        let floor = FactoryFloor::from(input);

        assert_eq!(
            floor.vertical_walls,
            HashSet::from([
                Coordinate { x: 7, y: 1 },
                Coordinate { x: 7, y: 2 },
                Coordinate { x: 11, y: 1 },
                Coordinate { x: 11, y: 2 },
                Coordinate { x: 11, y: 3 },
                Coordinate { x: 11, y: 4 },
                Coordinate { x: 11, y: 5 },
                Coordinate { x: 11, y: 6 },
                Coordinate { x: 9, y: 6 },
                Coordinate { x: 9, y: 5 },
                Coordinate { x: 2, y: 3 },
                Coordinate { x: 2, y: 4 },
            ])
        );
    }

    #[rstest]
    fn test_internal_points() {
        // Shape reminder:
        // ..............
        // .......#XXX#..
        // .......X...X..
        // ..#XXXX#...X..
        // ..X........X..
        // ..#XXXXXX#.X..
        // .........X.X..
        // .........#X#..
        // ..............

        // Internal points:
        // ..............
        // .......XXXXX..
        // .......XXXXX..
        // ..XXXXXXXXXX..
        // ..XXXXXXXXXX..
        // ..XXXXXXXXXX..
        // .........XXX..
        // .........XXX..
        // ..............

        let input = "7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3";
        let floor = FactoryFloor::from(input);

        let internal: HashSet<Coordinate> = (0..15)
            .flat_map(|x| (0..15).map(move |y| Coordinate { x, y }))
            .filter(|&point| floor.point_inside(point))
            .collect();

        assert_eq!(
            internal,
            HashSet::from([
                // Vertical
                Coordinate { x: 7, y: 1 },
                Coordinate { x: 7, y: 2 },
                Coordinate { x: 7, y: 3 },
                Coordinate { x: 11, y: 1 },
                Coordinate { x: 11, y: 2 },
                Coordinate { x: 11, y: 3 },
                Coordinate { x: 11, y: 4 },
                Coordinate { x: 11, y: 5 },
                Coordinate { x: 11, y: 6 },
                Coordinate { x: 11, y: 7 },
                Coordinate { x: 9, y: 7 },
                Coordinate { x: 9, y: 6 },
                Coordinate { x: 9, y: 5 },
                Coordinate { x: 2, y: 3 },
                Coordinate { x: 2, y: 4 },
                Coordinate { x: 2, y: 5 },
                // Horizontal
                Coordinate { x: 8, y: 1 },
                Coordinate { x: 9, y: 1 },
                Coordinate { x: 10, y: 1 },
                Coordinate { x: 3, y: 3 },
                Coordinate { x: 4, y: 3 },
                Coordinate { x: 5, y: 3 },
                Coordinate { x: 6, y: 3 },
                Coordinate { x: 3, y: 5 },
                Coordinate { x: 4, y: 5 },
                Coordinate { x: 5, y: 5 },
                Coordinate { x: 6, y: 5 },
                Coordinate { x: 7, y: 5 },
                Coordinate { x: 8, y: 5 },
                Coordinate { x: 5, y: 5 },
                Coordinate { x: 10, y: 7 },
                // Inside
                Coordinate { x: 8, y: 2 },
                Coordinate { x: 9, y: 2 },
                Coordinate { x: 10, y: 2 },
                Coordinate { x: 8, y: 3 },
                Coordinate { x: 9, y: 3 },
                Coordinate { x: 10, y: 3 },
                Coordinate { x: 3, y: 4 },
                Coordinate { x: 4, y: 4 },
                Coordinate { x: 5, y: 4 },
                Coordinate { x: 6, y: 4 },
                Coordinate { x: 7, y: 4 },
                Coordinate { x: 8, y: 4 },
                Coordinate { x: 9, y: 4 },
                Coordinate { x: 10, y: 4 },
                Coordinate { x: 3, y: 5 },
                Coordinate { x: 4, y: 5 },
                Coordinate { x: 5, y: 5 },
                Coordinate { x: 6, y: 5 },
                Coordinate { x: 7, y: 5 },
                Coordinate { x: 8, y: 5 },
                Coordinate { x: 9, y: 5 },
                Coordinate { x: 10, y: 5 },
                Coordinate { x: 10, y: 6 },
            ])
        );
    }
}
