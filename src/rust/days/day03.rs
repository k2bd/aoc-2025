use std::ops::Index;

use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

#[pymodule(module = "aoc_2025.rs.day03")]
pub fn day3(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<BatteryBank>()?;

    Ok(())
}

#[gen_stub_pyclass]
#[pyclass(module = "aoc_2025.rs.day03")]
struct BatteryBank {
    batteries: Vec<u8>,
}

impl From<&str> for BatteryBank {
    fn from(value: &str) -> Self {
        Self {
            batteries: value
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect(),
        }
    }
}

impl BatteryBank {
    /// Get (pos, value) of the first instance of the maximum value
    /// in a slice of the batteries
    fn max_of_section(&self, from: usize, to: usize) -> (usize, u8) {
        let &max = self.batteries[from..to].iter().max().unwrap();
        let (ind, _) = self.batteries[from..to]
            .iter()
            .enumerate()
            .find(|(_, &v)| v == max)
            .unwrap();
        (ind, max)
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl BatteryBank {
    #[new]
    fn new(battery: &str) -> Self {
        Self::from(battery)
    }

    fn joltage(&self) -> usize {
        let (max_ind, max) = self.max_of_section(0, self.batteries.len() - 1);
        let (_, next) = self.max_of_section(max_ind + 1, self.batteries.len());

        (max.to_string() + &next.to_string()).parse().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("987654321111111", 98)]
    #[case("811111111111119", 89)]
    #[case("234234234234278", 78)]
    #[case("818181911112111", 92)]
    #[case("818181911112111", 92)]
    #[case("54321", 54)]
    #[case("12345", 45)]
    #[case("111111", 11)]
    #[case("19", 19)]
    #[case("91", 91)]
    #[case("8781", 88)]
    fn test_joltage(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(BatteryBank::from(input).joltage(), expected)
    }
}
