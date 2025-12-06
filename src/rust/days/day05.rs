use std::collections::HashSet;

use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

#[pymodule(module = "aoc_2025.rs.day05")]
pub fn day5(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<KitchenIMS>()?;

    Ok(())
}

#[gen_stub_pyclass]
#[pyclass(module = "aoc_2025.rs.day05")]
struct KitchenIMS {
    fresh: Vec<(usize, usize)>,
    available: HashSet<usize>,
}

impl From<&str> for KitchenIMS {
    fn from(value: &str) -> Self {
        Self {
            fresh: value
                .lines()
                .take_while(|line| !line.is_empty())
                .map(|line| {
                    let (left, right) = line.split_once("-").unwrap();
                    (left.parse().unwrap(), right.parse().unwrap())
                })
                .collect(),
            available: value
                .lines()
                .skip_while(|line| !line.is_empty())
                .skip(1)
                .map(|line| line.parse().unwrap())
                .collect(),
        }
    }
}

/// Take a set of overlapping ranges and produce an equivalent set of
/// non-overlapping ranges
fn consolidate_ranges(ranges: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut old_ranges = ranges.to_owned();
    let mut new_ranges = Vec::new();

    while let Some((base_min, base_max)) = old_ranges.pop() {
        let mut overlapping_indices: Vec<usize> = Vec::new();
        {
            old_ranges
                .iter()
                .enumerate()
                .filter_map(|(index, &(min, max))| {
                    if (min >= base_min && min <= base_max) || (max >= base_min && max <= base_max)
                    {
                        Some(index)
                    } else {
                        None
                    }
                })
                .rev()
                .for_each(|index| overlapping_indices.push(index));
        }
        let mut to_combine = vec![(base_min, base_max)];
        for index in overlapping_indices {
            to_combine.push(old_ranges.remove(index));
        }

        let combined_min = to_combine.iter().map(|&(min, _)| min).min().unwrap();
        let combined_max = to_combine.iter().map(|&(_, max)| max).max().unwrap();

        new_ranges.push((combined_min, combined_max));
    }

    if new_ranges.len() != ranges.len() {
        new_ranges = consolidate_ranges(&new_ranges);
    }

    new_ranges
}

#[gen_stub_pymethods]
#[pymethods]
impl KitchenIMS {
    #[new]
    fn new(value: &str) -> Self {
        Self::from(value)
    }

    fn available_and_fresh(&self) -> HashSet<usize> {
        self.available
            .iter()
            .filter(|&&id| self.fresh.iter().any(|&(min, max)| id >= min && id <= max))
            .cloned()
            .collect()
    }

    fn fresh_ids_count(&self) -> usize {
        consolidate_ranges(&self.fresh)
            .iter()
            .map(|(min, max)| max + 1 - min)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    fn test_create_kitchen_ims() {
        let example_input = "3-5\n10-14\n\n1\n5";
        let ims = KitchenIMS::from(example_input);
        assert_eq!(ims.fresh, vec![(3, 5), (10, 14)]);
        assert_eq!(ims.available, HashSet::from([1, 5]));
    }

    #[rstest]
    fn test_available_and_fresh() {
        let example_input = "3-5\n10-14\n\n1\n5";
        let ims = KitchenIMS::from(example_input);
        assert_eq!(ims.available_and_fresh(), HashSet::from([5]));
    }

    #[rstest]
    fn test_consolidate_ranges() {
        let ranges = vec![
            (1, 5),
            (4, 10),
            (8, 15),
            (20, 30),
            (30, 40),
            (50, 60),
            (102, 105),
            (101, 107),
            (105, 106),
        ];

        assert_eq!(
            consolidate_ranges(&ranges),
            vec![(101, 107), (50, 60), (20, 40), (1, 15)]
        );
    }
}
