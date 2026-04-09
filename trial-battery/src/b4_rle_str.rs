pub fn run_length_encode_str(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }

    let mut chars = input.chars();
    let mut current = chars.next().unwrap();
    let mut count = 1usize;
    let mut encoded = String::new();

    for ch in chars {
        if ch == current {
            count += 1;
        } else {
            encoded.push(current);
            encoded.push_str(&count.to_string());
            current = ch;
            count = 1;
        }
    }

    encoded.push(current);
    encoded.push_str(&count.to_string());
    encoded
}

#[cfg(test)]
mod tests {
    use super::run_length_encode_str;

    #[test]
    fn simple() {
        assert_eq!(run_length_encode_str("aaabbc"), "a3b2c1");
    }

    #[test]
    fn single() {
        assert_eq!(run_length_encode_str("x"), "x1");
    }

    #[test]
    fn empty() {
        assert_eq!(run_length_encode_str(""), "");
    }

    #[test]
    fn long_run() {
        assert_eq!(run_length_encode_str("aaaaaaaaaa"), "a10");
    }

    #[test]
    fn alternating() {
        assert_eq!(run_length_encode_str("ababab"), "a1b1a1b1a1b1");
    }
}
