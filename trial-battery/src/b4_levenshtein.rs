pub fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let m = a_chars.len();
    let n = b_chars.len();

    if m == 0 {
        return n;
    }
    if n == 0 {
        return m;
    }

    let mut dp = vec![vec![0usize; n + 1]; m + 1];

    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }

    for i in 1..=m {
        for j in 1..=n {
            let cost = if a_chars[i - 1] == b_chars[j - 1] { 0 } else { 1 };
            dp[i][j] = (dp[i - 1][j] + 1)
                .min(dp[i][j - 1] + 1)
                .min(dp[i - 1][j - 1] + cost);
        }
    }

    dp[m][n]
}

#[cfg(test)]
mod tests {
    use super::levenshtein_distance;

    #[test]
    fn identical() {
        assert_eq!(levenshtein_distance("kitten", "kitten"), 0);
    }

    #[test]
    fn classic() {
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
    }

    #[test]
    fn empty_a() {
        assert_eq!(levenshtein_distance("", "abc"), 3);
    }

    #[test]
    fn empty_b() {
        assert_eq!(levenshtein_distance("xyz", ""), 3);
    }

    #[test]
    fn both_empty() {
        assert_eq!(levenshtein_distance("", ""), 0);
    }
}
