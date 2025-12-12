use pyo3::prelude::*;
use pyo3_stub_gen::derive::gen_stub_pyfunction;

#[pymodule(module = "aoc_2026.rs.day09")]
pub fn day9(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(largest_carpet_area, m)?)?;

    Ok(())
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Clone, Copy, Debug)]
struct Coordinate(isize, isize);

fn area(p1: Coordinate, p2: Coordinate) -> usize {
    (((p1.0 - p2.0).abs() + 1) * ((p1.1 - p2.1).abs() + 1)) as usize
}

#[gen_stub_pyfunction(module = "aoc_2025.rs.day09")]
#[pyfunction]
fn largest_carpet_area(input: &str) -> usize {
    let points = input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            Coordinate(x.parse().unwrap(), y.parse().unwrap())
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
