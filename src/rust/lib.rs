use pyo3::{prelude::*, wrap_pymodule};
use pyo3_stub_gen::define_stub_info_gatherer;
mod days;

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn rs(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Day 1
    let day1_sub = wrap_pymodule!(days::day01::day1);
    m.add_wrapped(day1_sub)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("aoc_2025.rs.day01", day1_sub(py))?;

    // Day 2
    let day2_sub = wrap_pymodule!(days::day02::day2);
    m.add_wrapped(day2_sub)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("aoc_2025.rs.day02", day2_sub(py))?;

    Ok(())
}

define_stub_info_gatherer!(stub_info);
