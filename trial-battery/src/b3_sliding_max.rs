pub fn sliding_window_max(values: &[i64], k: usize) -> Vec<i64> {
    if k == 0 || values.len() < k {
        return Vec::new();
    }

    let mut result = Vec::with_capacity(values.len() - k + 1);
    for i in 0..=values.len() - k {
        result.push(values[i..i + k].iter().max().copied().unwrap());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::sliding_window_max;

    #[test]
    fn standard_window() {
        assert_eq!(
            sliding_window_max(&[1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![3, 3, 5, 5, 6, 7]
        );
    }

    #[test]
    fn window_of_one() {
        assert_eq!(sliding_window_max(&[5, 2, 8, 1], 1), vec![5, 2, 8, 1]);
    }

    #[test]
    fn window_equals_len() {
        assert_eq!(sliding_window_max(&[1, 2, 3], 3), vec![3]);
    }

    #[test]
    fn empty_input() {
        assert_eq!(sliding_window_max(&[], 3), Vec::<i64>::new());
    }

    #[test]
    fn k_larger_than_input() {
        assert_eq!(sliding_window_max(&[1, 2], 5), Vec::<i64>::new());
    }
}
