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
    /// Get (index, value) of the first instance of the maximum value in a
    /// section of the battery bank.
    ///
    /// ``start`` is inclusive; ``end`` is exclusive.
    fn max_of_section(&self, from: usize, to: usize) -> (usize, u8) {
        let &max = self.batteries[from..to].iter().max().unwrap();
        let (ind, _) = self.batteries[from..to]
            .iter()
            .enumerate()
            .find(|(_, &v)| v == max)
            .unwrap();
        (from + ind, max)
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl BatteryBank {
    #[new]
    fn new(battery: &str) -> Self {
        Self::from(battery)
    }

    /// Find the max joltage of the battery bank using the given number of batteries
    fn joltage(&self, using: usize) -> usize {
        let mut start = 0;
        let mut end = self.batteries.len() - using + 1;
        let mut result = String::new();

        for _ in 0..using {
            let (index, max) = self.max_of_section(start, end);
            start = index + 1;
            end += 1;
            result += &max.to_string();
        }

        result.parse().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("818181911112111", 0, 14, (6, 9))]
    #[case("818181911112111", 0, 3, (0, 8))]
    #[case("818181911112111", 7, 14, (11, 2))]
    #[case("8781", 0, 4, (0, 8))]
    fn test_max_of_section(
        #[case] input: &str,
        #[case] from: usize,
        #[case] to: usize,
        #[case] expected: (usize, u8),
    ) {
        assert_eq!(BatteryBank::from(input).max_of_section(from, to), expected);
    }

    #[rstest]
    #[case("987654321111111", 98)]
    #[case("811111111111119", 89)]
    #[case("234234234234278", 78)]
    #[case("818181911112111", 92)]
    #[case("54321", 54)]
    #[case("12345", 45)]
    #[case("111111", 11)]
    #[case("19", 19)]
    #[case("91", 91)]
    #[case("8781", 88)]
    fn test_joltage_2(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(BatteryBank::from(input).joltage(2), expected)
    }

    #[rstest]
    #[case("987654321111111", 987654321111)]
    #[case("811111111111119", 811111111119)]
    #[case("234234234234278", 434234234278)]
    #[case("818181911112111", 888911112111)]
    fn test_joltage_12(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(BatteryBank::from(input).joltage(12), expected)
    }
}
