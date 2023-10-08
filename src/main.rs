use sha2::Digest;
use thiserror::Error;

const DATA_LENGTH: usize = 12;
const CHECKSUM_LENGTH: usize = 8;
const TOTAL_LENGTH: usize = DATA_LENGTH + CHECKSUM_LENGTH;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Error)]
pub enum GenerateError {
    #[error("Prefix can include only valid base32 characters (a-z, 2-7)")]
    InvalidCharacter,
    #[error("Prefix has to start with a letter")]
    InvalidFirstCharacter,
}

pub fn generate_with_prefix(prefix: &str) -> Result<String, GenerateError> {
    let mut prefix = prefix.to_ascii_uppercase();
    let prefix_bits = prefix.len() * 5; // each base32 character encodes 5 bits
    if prefix_bits % 8 != 0 {
        // pad with 'A' (decodes as 0) to full bytes so that extra bits don't change the character
        let bits_missing_to_full_byte = 8 - prefix_bits % 8;
        let characters_missing_to_full_byte = bits_missing_to_full_byte.div_ceil(5);
        for _ in 0..characters_missing_to_full_byte {
            prefix.push('A');
        }
    }
    let prefix = prefix.as_bytes();

    // validate that prefix is a valid base32 encoding
    if !prefix
        .iter()
        .all(|b| b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567".contains(b))
    {
        return Err(GenerateError::InvalidCharacter);
    }
    if prefix.first().map(u8::is_ascii_uppercase) == Some(false) {
        return Err(GenerateError::InvalidFirstCharacter);
    }

    let prefix = data_encoding::BASE32_NOPAD
        .decode(prefix)
        .expect("prefix should have been validated by this point");

    let mut data: [u8; DATA_LENGTH] = rand::random();
    for (d, p) in data.iter_mut().zip(prefix.into_iter()) {
        *d = p;
    }
    let checksum: [u8; CHECKSUM_LENGTH] = sha2::Sha256::digest(data)[0..CHECKSUM_LENGTH]
        .try_into()
        .unwrap();

    let mut bytes = [0u8; TOTAL_LENGTH];
    bytes[0..DATA_LENGTH].copy_from_slice(&data);
    bytes[DATA_LENGTH..].copy_from_slice(&checksum);

    Ok(data_encoding::BASE32_NOPAD
        .encode(&bytes)
        .to_ascii_lowercase())
}

fn main() {
    for prefix in std::env::args().skip(1) {
        println!("{}: {}", prefix, generate_with_prefix(&prefix).unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        for case in ["", "aa", "x", "X", "foo", "foobar"] {
            match generate_with_prefix(case) {
                Ok(generated) => {
                    assert_eq!(&case.to_ascii_lowercase(), &generated[0..case.len()]);
                }
                Err(err) => {
                    panic!("Failed to generate for: {:?} Error: {:?}", case, err);
                }
            }
        }
    }

    #[test]
    fn test_generate_too_long() {
        let generated = generate_with_prefix(
            "toolongtoevenfitinthegeneratedpackageidbutwhocaresitshouldbetrimmed",
        )
        .unwrap();
        let actual_prefix = "toolongtoevenfitint";
        assert_eq!(&generated[0..actual_prefix.len()], actual_prefix);
    }

    #[test]
    fn test_failures() {
        assert_eq!(
            generate_with_prefix("2isnotavalidfirstcharacter"),
            Err(GenerateError::InvalidFirstCharacter)
        );
        assert_eq!(
            generate_with_prefix("thereisno1inbase32"),
            Err(GenerateError::InvalidCharacter)
        );
        assert_eq!(
            generate_with_prefix("dontpad="),
            Err(GenerateError::InvalidCharacter)
        );
    }
}
