use pyo3::{prelude::*, wrap_pymodule};
use pyo3_stub_gen::{define_stub_info_gatherer, derive::gen_stub_pyfunction};
mod days;

// Base module unfortunately requires some content to generate __init__.pyi
// See https://github.com/Jij-Inc/pyo3-stub-gen/issues/107
#[gen_stub_pyfunction]
#[pyfunction]
fn merry_christmas() {
    println!("Merry Christmas 2025!")
}

#[pymodule]
fn rs(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(merry_christmas, m)?)?;

    let day1_sub = wrap_pymodule!(days::day01::day1);
    m.add_wrapped(day1_sub)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("aoc_2025.rs.day01", day1_sub(py))?;

    let day2_sub = wrap_pymodule!(days::day02::day2);
    m.add_wrapped(day2_sub)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("aoc_2025.rs.day02", day2_sub(py))?;

    let day3_sub = wrap_pymodule!(days::day03::day3);
    m.add_wrapped(day3_sub)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("aoc_2025.rs.day03", day3_sub(py))?;

    let day4_sub = wrap_pymodule!(days::day04::day4);
    m.add_wrapped(day4_sub)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("aoc_2025.rs.day04", day4_sub(py))?;

    let day5_sub = wrap_pymodule!(days::day05::day5);
    m.add_wrapped(day5_sub)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("aoc_2025.rs.day05", day5_sub(py))?;

    let day6_sub = wrap_pymodule!(days::day06::day6);
    m.add_wrapped(day6_sub)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("aoc_2025.rs.day06", day6_sub(py))?;

    let day7_sub = wrap_pymodule!(days::day07::day7);
    m.add_wrapped(day7_sub)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("aoc_2025.rs.day07", day7_sub(py))?;

    Ok(())
}

define_stub_info_gatherer!(stub_info);
