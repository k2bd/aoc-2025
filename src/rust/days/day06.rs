use core::panic;

use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

#[pymodule(module = "aoc_2026.rs.day06")]
pub fn day6(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Homework>()?;

    Ok(())
}

#[derive(PartialEq, Debug)]
enum Operation {
    Add,
    Mult,
}

#[derive(PartialEq, Debug)]
struct Problem {
    values: Vec<isize>,
    operation: Operation,
}

impl Problem {
    fn solve(&self) -> isize {
        match self.operation {
            Operation::Add => self.values.iter().sum(),
            Operation::Mult => self.values.iter().fold(1, |acc, &v| acc * v),
        }
    }
}

#[gen_stub_pyclass]
#[pyclass(module = "aoc_2025.rs.day06")]
#[derive(PartialEq, Debug)]
struct Homework {
    problems: Vec<Problem>,
}

impl From<&str> for Homework {
    fn from(value: &str) -> Self {
        let mut lines: Vec<&str> = value.lines().collect();

        let operation_line = lines.pop().unwrap();

        let transposed: Vec<Vec<isize>> = lines
            .iter()
            .map(|&line| {
                line.split_whitespace()
                    .map(|v| v.parse().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();
        if transposed.len() == 0 {
            panic!("No problem values");
        }

        let problem_values: Vec<Vec<isize>> = (0..transposed[0].len())
            .map(|col| {
                (0..transposed.len())
                    .map(|row| transposed[row][col])
                    .collect()
            })
            .collect();

        let operations = operation_line.split_whitespace().map(|v| match v {
            "+" => Operation::Add,
            "*" => Operation::Mult,
            _ => panic!("Invalid operation"),
        });

        Homework {
            problems: problem_values
                .into_iter()
                .zip(operations)
                .map(|(values, operation)| Problem { values, operation })
                .collect(),
        }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl Homework {
    #[new]
    fn new(value: &str) -> Self {
        Self::from(value)
    }

    fn grand_total(&self) -> isize {
        self.problems.iter().map(|p| p.solve()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    fn test_create_homework() {
        let example_input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
        let homework = Homework::from(example_input);

        assert_eq!(
            homework,
            Homework {
                problems: vec![
                    Problem {
                        values: vec![123, 45, 6],
                        operation: Operation::Mult,
                    },
                    Problem {
                        values: vec![328, 64, 98],
                        operation: Operation::Add,
                    },
                    Problem {
                        values: vec![51, 387, 215],
                        operation: Operation::Mult,
                    },
                    Problem {
                        values: vec![64, 23, 314],
                        operation: Operation::Add,
                    },
                ]
            }
        )
    }
}
