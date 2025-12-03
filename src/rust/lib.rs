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

    Ok(())
}

define_stub_info_gatherer!(stub_info);
