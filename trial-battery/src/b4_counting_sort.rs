pub fn counting_sort(input: &[i64]) -> Vec<i64> {
    if input.is_empty() {
        return Vec::new();
    }

    let mut counts = vec![0u64; 2001];

    for &v in input {
        if v < -1000 || v > 1000 {
            panic!("counting_sort: out of range");
        }
        counts[(v + 1000) as usize] += 1;
    }

    let mut result = Vec::with_capacity(input.len());

    for (i, &c) in counts.iter().enumerate() {
        for _ in 0..c {
            result.push(i as i64 - 1000);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::counting_sort;

    #[test]
    fn sort_simple() {
        assert_eq!(
            counting_sort(&[3, 1, 4, 1, 5, 9, 2, 6, 5, 3]),
            vec![1, 1, 2, 3, 3, 4, 5, 5, 6, 9]
        );
    }

    #[test]
    fn sort_negative() {
        assert_eq!(counting_sort(&[-5, 3, -1, 0, 2]), vec![-5, -1, 0, 2, 3]);
    }

    #[test]
    fn sort_empty() {
        assert_eq!(counting_sort(&[]), Vec::<i64>::new());
    }

    #[test]
    fn sort_one() {
        assert_eq!(counting_sort(&[42]), vec![42]);
    }

    #[test]
    fn sort_already_sorted() {
        assert_eq!(counting_sort(&[1, 2, 3]), vec![1, 2, 3]);
    }
}
