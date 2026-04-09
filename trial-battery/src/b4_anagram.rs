pub fn is_anagram(a: &str, b: &str) -> bool {
    let mut a_chars: Vec<char> = a
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    let mut b_chars: Vec<char> = b
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    a_chars.sort();
    b_chars.sort();

    a_chars == b_chars
}

#[cfg(test)]
mod tests {
    use super::is_anagram;

    #[test]
    fn classic() {
        assert!(is_anagram("listen", "silent"));
    }

    #[test]
    fn case_insensitive() {
        assert!(is_anagram("Tea", "Eat"));
    }

    #[test]
    fn ignores_whitespace() {
        assert!(is_anagram("conversation", "voices rant on"));
    }

    #[test]
    fn not_anagram() {
        assert!(!is_anagram("hello", "world"));
    }

    #[test]
    fn empty_strings() {
        assert!(is_anagram("", ""));
    }
}
