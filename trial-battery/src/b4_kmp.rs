pub fn kmp_find_all(text: &str, pattern: &str) -> Vec<usize> {
    if pattern.is_empty() {
        return Vec::new();
    }

    let text_bytes = text.as_bytes();
    let pattern_bytes = pattern.as_bytes();
    let lps = build_lps(pattern_bytes);
    let mut result = Vec::new();

    let mut i = 0;
    let mut j = 0;

    while i < text_bytes.len() {
        if text_bytes[i] == pattern_bytes[j] {
            i += 1;
            j += 1;

            if j == pattern_bytes.len() {
                result.push(i - j);
                j = lps[j - 1];
            }
        } else if j != 0 {
            j = lps[j - 1];
        } else {
            i += 1;
        }
    }

    result
}

fn build_lps(pattern: &[u8]) -> Vec<usize> {
    let mut lps = vec![0; pattern.len()];
    let mut len = 0;
    let mut i = 1;

    while i < pattern.len() {
        if pattern[i] == pattern[len] {
            len += 1;
            lps[i] = len;
            i += 1;
        } else if len != 0 {
            len = lps[len - 1];
        } else {
            lps[i] = 0;
            i += 1;
        }
    }

    lps
}

#[cfg(test)]
mod tests {
    use super::kmp_find_all;

    #[test]
    fn find_multiple() {
        assert_eq!(kmp_find_all("abcabcabcabc", "abc"), vec![0, 3, 6, 9]);
    }

    #[test]
    fn find_single() {
        assert_eq!(kmp_find_all("hello world", "world"), vec![6]);
    }

    #[test]
    fn no_match() {
        assert_eq!(kmp_find_all("abcdef", "xyz"), Vec::<usize>::new());
    }

    #[test]
    fn empty_pattern() {
        assert_eq!(kmp_find_all("anything", ""), Vec::<usize>::new());
    }

    #[test]
    fn overlapping() {
        assert_eq!(kmp_find_all("aaaaa", "aa"), vec![0, 1, 2, 3]);
    }
}
