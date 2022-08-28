use wpkg_key::key;

pub const KEY: usize = key!();

pub fn _encode(key: usize, value: &str) -> String {
    let value_bytes = value.as_bytes();

    let mut output = Vec::new();

    for byte in value_bytes {
        let byte = *byte as usize;

        let byte_out = byte * key;

        output.push(byte_out.to_string());
    }

    output.join(" ")
}

pub fn decode(value: &str) -> String {
    _decode_key(KEY, value)
}

pub fn _decode_key(key: usize, value: &str) -> String {
    let mut output = Vec::new();

    let bytes: Vec<&str> = value.split_ascii_whitespace().collect();

    for byte in bytes {
        let byte: usize = byte.parse().unwrap();

        let out = byte / key;

        output.push(out as u8);
    }

    String::from_utf8(output).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(key: usize) {
        let value = "1234567890qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM~!@#$%^&*()_+-={}|[]\\:;\"',./<>?";

        let output_enc = _encode(key, value);

        let output_dec = _decode_key(key, &output_enc);

        assert_eq!(output_dec, value)
    }

    #[test]
    fn test() {
        for i in 1..u16::MAX {
            use std::time::Instant;

            let before = Instant::now();
            check(i as usize);
            println!("Elapsed time: {:.2?}", before.elapsed());
        }
    }
}
