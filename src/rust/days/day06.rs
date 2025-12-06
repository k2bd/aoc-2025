use core::panic;

use pyo3::{prelude::*, types::PyType};
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
            Operation::Mult => self.values.iter().product(),
        }
    }
}

#[gen_stub_pyclass]
#[pyclass(module = "aoc_2025.rs.day06")]
#[derive(PartialEq, Debug)]
struct Homework {
    problems: Vec<Problem>,
}

#[gen_stub_pymethods]
#[pymethods]
impl Homework {
    #[classmethod]
    #[pyo3(signature = (value))]
    fn read_wrong(_cls: &Bound<'_, PyType>, value: &str) -> Self {
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
        if transposed.is_empty() {
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

        Self {
            problems: problem_values
                .into_iter()
                .zip(operations)
                .map(|(values, operation)| Problem { values, operation })
                .collect(),
        }
    }

    #[classmethod]
    #[pyo3(signature = (value))]
    fn read_right(_cls: &Bound<'_, PyType>, value: &str) -> Self {
        let transposed_chars: Vec<Vec<char>> =
            value.lines().map(|line| line.chars().collect()).collect();
        if transposed_chars.is_empty() {
            panic!("No problem values");
        }
        let (numerical_input, operation_input): (Vec<String>, Vec<String>) =
            (0..transposed_chars[0].len())
                .map(|col| {
                    let str = (0..transposed_chars.len())
                        .map(|row| transposed_chars[row][col])
                        .collect::<String>();
                    let (num_part, op_part) = str.split_at(str.len() - 1);
                    (num_part.to_owned(), op_part.to_owned())
                })
                .collect();
        let operations: Vec<Operation> = operation_input
            .into_iter()
            .filter_map(|s| {
                if s.trim().is_empty() {
                    None
                } else {
                    match s.as_str() {
                        "+" => Some(Operation::Add),
                        "*" => Some(Operation::Mult),
                        _ => panic!("Invalid operator"),
                    }
                }
            })
            .collect();
        let (mut problem_values, last) = numerical_input.into_iter().fold(
            (Vec::<Vec<isize>>::new(), Vec::<isize>::new()),
            |(mut result, mut current_problem), val| {
                let trimmed = val.trim();
                if trimmed.is_empty() {
                    result.push(current_problem);
                    current_problem = Vec::new();
                } else {
                    current_problem.push(trimmed.parse().unwrap());
                }
                (result, current_problem)
            },
        );
        problem_values.push(last);

        Self {
            problems: problem_values
                .into_iter()
                .zip(operations)
                .map(|(values, operation)| Problem { values, operation })
                .collect(),
        }
    }

    fn grand_total(&self) -> isize {
        self.problems.iter().map(|p| p.solve()).sum()
    }
}
