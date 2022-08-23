pub const KEY: u8 = 3;

pub fn _encode(key: u8, value: &str) -> String {
    let value_bytes = value.as_bytes();

    let mut output = Vec::new();

    for byte in value_bytes {
        let byte = *byte as usize;

        let byte_out = byte << key;

        output.push(format!("{}", byte_out));
    }

    output.join(" ")
}

pub fn decode(value: &str) -> String {
    let mut output = Vec::new();

    let bytes: Vec<&str> = value.split_ascii_whitespace().collect();

    for byte in bytes {
        let byte: u128 = byte.parse().unwrap();

        let out = byte >> KEY;

        output.push(out as u8);
    }

    String::from_utf8(output).unwrap()
}

pub fn decode_key(key: u8, value: &str) -> String {
    let mut output = Vec::new();

    let bytes: Vec<&str> = value.split_ascii_whitespace().collect();

    for byte in bytes {
        let byte: u128 = byte.parse().unwrap();

        let out = byte >> key;

        output.push(out as u8);
    }

    String::from_utf8(output).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(key: u8) {
        let value = "1234567890qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM~!@#$%^&*()_+-={}|[]\\:;\"',./<>?";

        println!("Key:     {}", key);

        println!("Input:   {:?} ({})", value.as_bytes(), value);

        let output_enc = _encode(key, value);

        println!("Encoded: {:?}", output_enc);

        let output_dec = decode_key(key, &output_enc);

        println!("Decoded: {:?} ({})", output_dec.as_bytes(), output_dec);

        assert_eq!(output_dec, value)
    }

    #[test]
    fn test() {
        for i in 1..50 {
            check(i);
        }
    }
}
