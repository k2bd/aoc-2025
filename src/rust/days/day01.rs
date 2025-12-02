use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

#[pymodule(module = "aoc_2025.rs.day01")]
pub fn day1(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Safe>()?;

    Ok(())
}

#[gen_stub_pyclass]
#[pyclass(module = "aoc_2025.rs.day01")]
struct Safe {
    #[pyo3(get)]
    positions: isize,

    #[pyo3(get)]
    dial: isize,
}

#[gen_stub_pymethods]
#[pymethods]
impl Safe {
    #[new]
    fn new(positions: isize, initial: isize) -> Self {
        Self {
            positions,
            dial: initial.rem_euclid(positions),
        }
    }

    /// Turn the safe dial, returning the number of times the dial passes 0
    fn turn(&mut self, by: isize) -> usize {
        let new_value = self.dial + by;
        let new_position = new_value.rem_euclid(self.positions);

        // Transpose the rotation to always be clockwise
        let times_past_zero = if by < 0 {
            (if self.dial != 0 {
                self.positions - self.dial
            } else {
                0
            }) + by.abs()
        } else {
            new_value
        } / self.positions;

        self.dial = new_position;

        times_past_zero as usize
    }

    /// Turn the safe dial, returning the number of times the dial passes 0
    fn resolve_instruction(&mut self, instruction: &str) -> usize {
        let (dir, size_str) = instruction.split_at(1);
        let mut turn_by = size_str.parse::<isize>().unwrap();
        if dir == "L" {
            turn_by *= -1;
        }

        self.turn(turn_by)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(15, -10, 5, 0)]
    #[case(5, -10, 95, 1)]
    #[case(95, 5, 0, 1)]
    #[case(95, 10, 5, 1)]
    fn test_turn_dial(
        #[case] initial: isize,
        #[case] by: isize,
        #[case] expected_pos: isize,
        #[case] expected_past_zero: usize,
    ) {
        let mut safe = Safe::new(100, initial);
        assert_eq!(safe.turn(by), expected_past_zero);
        assert_eq!(safe.dial, expected_pos);
    }

    #[rstest]
    #[case(15, "L10", 5, 0)]
    #[case(5, "L10", 95, 1)]
    #[case(95, "R5", 0, 1)]
    #[case(5, "L5", 0, 1)]
    #[case(95, "R10", 5, 1)]
    #[case(95, "R210", 5, 3)]
    #[case(50, "R1000", 50, 10)]
    #[case(95, "L210", 85, 2)]
    #[case(50, "L1000", 50, 10)]
    #[case(0, "L1000", 0, 10)]
    #[case(0, "R1000", 0, 10)]
    #[case(0, "R100", 0, 1)]
    #[case(0, "L100", 0, 1)]
    #[case(0, "R5", 5, 0)]
    #[case(0, "L101", 99, 1)]
    #[case(1, "L101", 0, 2)]
    #[case(1, "L102", 99, 2)]
    #[case(0, "R101", 1, 1)]
    #[case(99, "R101", 0, 2)]
    #[case(99, "R102", 1, 2)]
    // Examples from site
    #[case(50, "L68", 82, 1)]
    #[case(82, "L30", 52, 0)]
    #[case(52, "R48", 0, 1)]
    #[case(0, "L5", 95, 0)]
    #[case(95, "R60", 55, 1)]
    #[case(55, "L55", 0, 1)]
    #[case(0, "L1", 99, 0)]
    #[case(99, "L99", 0, 1)]
    #[case(0, "R14", 14, 0)]
    #[case(14, "L82", 32, 1)]
    fn test_resolve_instruction(
        #[case] initial: isize,
        #[case] instruction: &str,
        #[case] expected_pos: isize,
        #[case] expected_past_zero: usize,
    ) {
        let mut safe = Safe::new(100, initial);
        let passes = safe.resolve_instruction(instruction);
        assert_eq!(safe.dial, expected_pos);
        assert_eq!(passes, expected_past_zero);
    }
}
