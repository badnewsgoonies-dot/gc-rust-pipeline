pub fn base64_encode(input: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut output = String::with_capacity(input.len().div_ceil(3) * 4);

    for chunk in input.chunks(3) {
        match chunk.len() {
            3 => {
                let n = ((chunk[0] as u32) << 16) | ((chunk[1] as u32) << 8) | (chunk[2] as u32);
                output.push(ALPHABET[((n >> 18) & 0x3F) as usize] as char);
                output.push(ALPHABET[((n >> 12) & 0x3F) as usize] as char);
                output.push(ALPHABET[((n >> 6) & 0x3F) as usize] as char);
                output.push(ALPHABET[(n & 0x3F) as usize] as char);
            }
            2 => {
                let n = ((chunk[0] as u32) << 16) | ((chunk[1] as u32) << 8);
                output.push(ALPHABET[((n >> 18) & 0x3F) as usize] as char);
                output.push(ALPHABET[((n >> 12) & 0x3F) as usize] as char);
                output.push(ALPHABET[((n >> 6) & 0x3F) as usize] as char);
                output.push('=');
            }
            1 => {
                let n = (chunk[0] as u32) << 16;
                output.push(ALPHABET[((n >> 18) & 0x3F) as usize] as char);
                output.push(ALPHABET[((n >> 12) & 0x3F) as usize] as char);
                output.push('=');
                output.push('=');
            }
            _ => {}
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::base64_encode;

    #[test]
    fn encode_empty() {
        assert_eq!(base64_encode(b""), "");
    }

    #[test]
    fn encode_one_byte() {
        assert_eq!(base64_encode(b"f"), "Zg==");
    }

    #[test]
    fn encode_two_bytes() {
        assert_eq!(base64_encode(b"fo"), "Zm8=");
    }

    #[test]
    fn encode_three_bytes() {
        assert_eq!(base64_encode(b"foo"), "Zm9v");
    }

    #[test]
    fn encode_man() {
        assert_eq!(base64_encode(b"Man"), "TWFu");
    }

    #[test]
    fn encode_hello() {
        assert_eq!(base64_encode(b"Hello"), "SGVsbG8=");
    }
}
