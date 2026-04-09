pub fn encode(input: &[u8]) -> Vec<(u8, u32)> {
    if input.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::new();
    let mut current = input[0];
    let mut count: u32 = 1;

    for &byte in &input[1..] {
        if byte == current {
            count += 1;
        } else {
            result.push((current, count));
            current = byte;
            count = 1;
        }
    }

    result.push((current, count));
    result
}

pub fn decode(pairs: &[(u8, u32)]) -> Vec<u8> {
    let total_len: usize = pairs.iter().map(|&(_, count)| count as usize).sum();
    let mut result = Vec::with_capacity(total_len);

    for &(byte, count) in pairs {
        result.extend(std::iter::repeat(byte).take(count as usize));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::{decode, encode};

    #[test]
    fn encode_simple() {
        assert_eq!(encode(&[1, 1, 1, 2, 2, 3]), vec![(1, 3), (2, 2), (3, 1)]);
    }

    #[test]
    fn encode_empty() {
        assert_eq!(encode(&[]), Vec::<(u8, u32)>::new());
    }

    #[test]
    fn encode_single() {
        assert_eq!(encode(&[42]), vec![(42, 1)]);
    }

    #[test]
    fn decode_simple() {
        assert_eq!(decode(&[(1, 3), (2, 2), (3, 1)]), vec![1, 1, 1, 2, 2, 3]);
    }

    #[test]
    fn round_trip() {
        let input = vec![5, 5, 5, 5, 10, 10, 20, 20, 20];
        assert_eq!(decode(&encode(&input)), input);
    }
}
