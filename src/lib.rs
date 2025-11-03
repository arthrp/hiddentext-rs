const ZERO: char = '\u{200C}';
const ONE: char = '\u{200D}';

pub fn encode(text: &str) -> String {
    let mut result = String::new();
    for &b in text.as_bytes() {
        for i in (0..8).rev() {
            if (b >> i) & 1 == 1 {
                result.push(ONE);
            } else {
                result.push(ZERO);
            }
        }
    }
    result
}

#[derive(Debug, PartialEq, Eq)]
pub enum DecodeError {
    InvalidLength,
    InvalidCharacter,
    InvalidUtf8,
}

pub fn decode(encoded_text: &str) -> Result<String, DecodeError> {
    if encoded_text.chars().count() % 8 != 0 {
        return Err(DecodeError::InvalidLength);
    }

    let mut bytes = Vec::new();
    let chars: Vec<char> = encoded_text.chars().collect();

    for chunk in chars.chunks(8) {
        let mut b = 0u8;
        for (i, &c) in chunk.iter().enumerate() {
            match c {
                ONE => b |= 1 << (7 - i),
                ZERO => (),
                _ => return Err(DecodeError::InvalidCharacter),
            }
        }
        bytes.push(b);
    }

    String::from_utf8(bytes).map_err(|_| DecodeError::InvalidUtf8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_works() {
        let text = "a"; // binary 01100001
        let mut expected = String::new();
        expected.push(ZERO); // 0
        expected.push(ONE);  // 1
        expected.push(ONE);  // 1
        expected.push(ZERO); // 0
        expected.push(ZERO); // 0
        expected.push(ZERO); // 0
        expected.push(ZERO); // 0
        expected.push(ONE);  // 1
        assert_eq!(encode(text), expected);
    }

    #[test]
    fn decode_works() {
        let mut encoded = String::new();
        encoded.push(ZERO);
        encoded.push(ONE);
        encoded.push(ONE);
        encoded.push(ZERO);
        encoded.push(ZERO);
        encoded.push(ZERO);
        encoded.push(ZERO);
        encoded.push(ONE);
        let expected = "a";
        assert_eq!(decode(&encoded).unwrap(), expected);
    }

    #[test]
    fn encode_decode_roundtrip_works() {
        let original = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";
        let encoded = encode(original);
        let decoded = decode(&encoded).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn encode_decode_emoji_works() {
        let original = "😀 café ₿";
        let encoded = encode(original);
        let decoded = decode(&encoded).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn decode_invalid_length_returns_error() {
        let encoded = "a";
        assert_eq!(decode(encoded), Err(DecodeError::InvalidLength));
    }

    #[test]
    fn decode_invalid_character_returns_error() {
        let encoded = "abcdefgh";
        assert_eq!(decode(encoded), Err(DecodeError::InvalidCharacter));
    }

    #[test]
    fn decode_invalid_utf8_returns_error() {
        let mut encoded = String::new();
        // This is an invalid UTF-8 sequence (specifically, a lone surrogate half).
        // 11011000 00000000 -> 0xD8 0x00
        for _ in 0..4 { encoded.push(ONE); }   // 1101
        for _ in 0..4 { encoded.push(ZERO); }  // 1000
        for _ in 0..8 { encoded.push(ZERO); } // 00000000
        assert_eq!(decode(&encoded), Err(DecodeError::InvalidUtf8));
    }
}
