#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    pub validation: bool,
    pub expected: String,
}

impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {
        CipherError {
            validation,
            expected,
        }
    }
}

pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {
    if ciphered.is_empty() || original.is_empty() {
        return None;
    }

    let expected = original
        .chars()
        .map(|c| match c {
            'a'..='z' => (b'z' - (c as u8 - b'a')) as char,
            'A'..='Z' => (b'Z' - (c as u8 - b'A')) as char,
            _ => c,
        })
        .collect::<String>();

    if expected == ciphered {
        Some(Ok(true))
    } else {
        Some(Err(CipherError::new(false, expected)))
    }
}
