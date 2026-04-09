use std::collections::HashMap;

pub fn char_frequency(input: &str) -> HashMap<char, u64> {
    let mut counts = HashMap::new();
    for ch in input.chars() {
        *counts.entry(ch).or_insert(0) += 1;
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::char_frequency;

    #[test]
    fn simple() {
        let f = char_frequency("hello");
        assert_eq!(f.get(&'h'), Some(&1));
        assert_eq!(f.get(&'e'), Some(&1));
        assert_eq!(f.get(&'l'), Some(&2));
        assert_eq!(f.get(&'o'), Some(&1));
    }

    #[test]
    fn empty() {
        let f = char_frequency("");
        assert!(f.is_empty());
    }

    #[test]
    fn unicode() {
        let f = char_frequency("aaä");
        assert_eq!(f.get(&'a'), Some(&2));
        assert_eq!(f.get(&'ä'), Some(&1));
    }

    #[test]
    fn whitespace_counted() {
        let f = char_frequency("a b c");
        assert_eq!(f.get(&' '), Some(&2));
    }
}
