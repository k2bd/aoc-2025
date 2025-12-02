use std::collections::HashSet;

use pyo3::prelude::*;
use pyo3_stub_gen::derive::gen_stub_pyfunction;

#[pymodule(module = "aoc_2025.rs.day02")]
pub fn day2(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_invalid_ids_p1, m)?)?;
    m.add_function(wrap_pyfunction!(get_invalid_ids_p2, m)?)?;

    Ok(())
}

fn halve_digits(val: usize) -> usize {
    if val < 10 {
        return 0;
    }
    let val_str = val.to_string();
    val_str[..val_str.len() / 2].parse().unwrap()
}

fn double_digits(val: usize) -> usize {
    let val_str = val.to_string();
    val_str.repeat(2).parse().unwrap()
}

#[gen_stub_pyfunction(module = "aoc_2025.rs.day02")]
#[pyfunction]
fn get_invalid_ids_p1(min: usize, max: usize) -> Vec<usize> {
    (halve_digits(min)..)
        .map(double_digits)
        .filter(|&value| value > 0)
        .filter(|&value| value >= min)
        .take_while(|&value| value <= max)
        .collect()
}

fn repeat_digits(val: usize, count: usize) -> usize {
    val.to_string().repeat(count).parse().unwrap()
}

fn value_in_ranges(val: usize, all_ranges: &[(usize, usize)]) -> bool {
    all_ranges
        .iter()
        .any(|&(min, max)| val >= min && val <= max)
}

#[gen_stub_pyfunction(module = "aoc_2025.rs.day02")]
#[pyfunction]
fn get_invalid_ids_p2(all_ranges: Vec<(usize, usize)>) -> HashSet<usize> {
    let biggest_max = all_ranges.iter().map(|&(_, max)| max).max().unwrap();

    (1..)
        .take_while(|&i| double_digits(i) <= biggest_max)
        .flat_map(|digit| {
            (2..)
                .map(move |count| repeat_digits(digit, count))
                .take_while(|&value| value <= biggest_max)
        })
        .filter(|&value| value_in_ranges(value, &all_ranges))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(11, 1)]
    #[case(101, 1)]
    #[case(1, 0)]
    fn test_halve_digits(#[case] val: usize, #[case] expected: usize) {
        assert_eq!(halve_digits(val), expected);
    }

    #[rstest]
    #[case(11, 1111)]
    #[case(101, 101101)]
    #[case(1, 11)]
    fn test_double_digits(#[case] val: usize, #[case] expected: usize) {
        assert_eq!(double_digits(val), expected);
    }

    #[rstest]
    #[case(11, 22, vec![11, 22])]
    #[case(95, 115, vec![99])]
    #[case(998, 1012, vec![1010])]
    #[case(1188511880, 1188511890, vec![1188511885])]
    #[case(222220, 222224, vec![222222])]
    #[case(1698522, 1698528, vec![])]
    #[case(446443, 446449, vec![446446])]
    #[case(38593856, 38593862, vec![38593859])]
    #[case(565653, 565659, vec![])]
    #[case(824824821, 824824827, vec![])]
    #[case(2121212118, 2121212124, vec![])]
    // My own tests
    #[case(0, 22, vec![11, 22])]
    fn test_get_invalid_ids_p1(
        #[case] min: usize,
        #[case] max: usize,
        #[case] expected_invalid_ids: Vec<usize>,
    ) {
        assert_eq!(get_invalid_ids_p1(min, max), expected_invalid_ids);
    }

    #[rstest]
    #[case(11, 2, 1111)]
    #[case(11, 3, 111111)]
    #[case(101, 2, 101101)]
    #[case(101, 3, 101101101)]
    #[case(1, 2, 11)]
    fn test_repeat_digits(#[case] val: usize, #[case] count: usize, #[case] expected: usize) {
        assert_eq!(repeat_digits(val, count), expected);
    }

    #[rstest]
    #[case(vec![(11, 22)], HashSet::from([11, 22]))]
    #[case(vec![(95, 115)], HashSet::from([99, 111]))]
    #[case(vec![(998, 1012)], HashSet::from([999, 1010]))]
    #[case(vec![(1188511880, 1188511890)], HashSet::from([1188511885]))]
    #[case(vec![(222220, 222224)], HashSet::from([222222]))]
    #[case(vec![(1698522, 1698528)], HashSet::from([]))]
    #[case(vec![(446443, 446449)], HashSet::from([446446]))]
    #[case(vec![(38593856, 38593862)], HashSet::from([38593859]))]
    #[case(vec![(565653, 565659)], HashSet::from([565656]))]
    #[case(vec![(824824821, 824824827)], HashSet::from([824824824]))]
    #[case(vec![(2121212118, 2121212124)], HashSet::from([2121212121]))]
    #[case(
        vec![
            (11, 22),
            (95, 115),
            (998, 1012),
            (1188511880, 1188511890),
            (222220, 222224),
            (1698522, 1698528),
            (446443, 446449),
            (38593856, 38593862),
            (565653, 565659),
            (824824821, 824824827),
            (2121212118, 2121212124),
        ],
        HashSet::from([11, 22, 99, 111, 999, 1010, 1188511885, 222222, 446446, 38593859, 565656, 824824824, 2121212121])
    )]
    // My own tests
    #[case(vec![(0, 22)], HashSet::from([11, 22]))]
    #[case(vec![(0, 22), (10, 33)], HashSet::from([11, 22, 33]))]
    fn test_get_invalid_ids_p2(
        #[case] ranges: Vec<(usize, usize)>,
        #[case] expected_invalid_ids: HashSet<usize>,
    ) {
        assert_eq!(get_invalid_ids_p2(ranges), expected_invalid_ids);
    }
}
