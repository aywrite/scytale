use caesar_shift::lower_caesar_shift;
use caesar_shift::upper_caesar_shift;
use text_cipher::TextCipher;

/// A vigenere cipher takes each character in the plaintext string and adds the character with the
/// same index from the key string to its alphanumeric value (mod 26) and converts it back to a new
/// character. If the end of the key is reached before the end of the message then the cipher
/// starts again from the begining of the key. "a" is treated as a shift of 0 and the case of the
/// key is ignored.
///
/// Only latin alphabet characters will be modified, any other characters, numbers, punctuation etc.
/// will be left as is. Case is also preserved.  A vigenere cipher takes each character in the input
#[derive(Debug, Clone)]
pub struct VigenereCipher {
    key: String,
}

impl VigenereCipher {
    pub fn new<T: Into<String>>(key: T) -> VigenereCipher {
        VigenereCipher { key: key.into() }
    }
}

impl TextCipher for VigenereCipher {
    fn encrypt(&self, text: &str) -> String {
        let mut result = String::new();
        for (c, k) in text.chars().zip(self.key.to_lowercase().chars().cycle()) {
            let shift = (k as u8) - 97;
            result.push(shift_char(c, shift));
        }
        result
    }

    fn decrypt(&self, text: &str) -> String {
        let mut result = String::new();
        for (c, k) in text.chars().zip(self.key.to_lowercase().chars().cycle()) {
            let shift = (26 - (k as u8 - 97)) % 26;
            result.push(shift_char(c, shift));
        }
        result
    }
}

fn shift_char(c: char, shift: u8) -> char {
    match c {
        'A'...'Z' => upper_caesar_shift(c, shift),
        'a'...'z' => lower_caesar_shift(c, shift),
        _ => c,
    }
}

#[cfg(test)]
mod encrypt_tests {

    use text_cipher::TextCipher;
    use vigenere::VigenereCipher;

    #[test]
    fn test_encrypt_lowercase() {
        let cipher = VigenereCipher::new("aaaa");
        assert_eq!("abyz", cipher.encrypt("abyz"));

        let cipher = VigenereCipher::new("AAAA");
        assert_eq!("abyz", cipher.encrypt("abyz"));

        let cipher = VigenereCipher::new("abcd");
        assert_eq!("acac", cipher.encrypt("abyz"));

        let cipher = VigenereCipher::new("ABCD");
        assert_eq!("acac", cipher.encrypt("abyz"));

        let cipher = VigenereCipher::new("abyz");
        assert_eq!("abyz", cipher.encrypt("aaaa"));

        let cipher = VigenereCipher::new("ABYZ");
        assert_eq!("abyz", cipher.encrypt("aaaa"));

        let cipher = VigenereCipher::new("abyz");
        assert_eq!("acac", cipher.encrypt("abcd"));

        let cipher = VigenereCipher::new("ABYZ");
        assert_eq!("acac", cipher.encrypt("abcd"));
    }

    #[test]
    fn test_encrypt_uppercase() {
        let cipher = VigenereCipher::new("aaaa");
        assert_eq!("ABYZ", cipher.encrypt("ABYZ"));

        let cipher = VigenereCipher::new("AAAA");
        assert_eq!("ABYZ", cipher.encrypt("ABYZ"));

        let cipher = VigenereCipher::new("abcd");
        assert_eq!("ACAC", cipher.encrypt("ABYZ"));

        let cipher = VigenereCipher::new("ABCD");
        assert_eq!("ACAC", cipher.encrypt("ABYZ"));

        let cipher = VigenereCipher::new("abyz");
        assert_eq!("ABYZ", cipher.encrypt("AAAA"));

        let cipher = VigenereCipher::new("ABYZ");
        assert_eq!("ABYZ", cipher.encrypt("AAAA"));

        let cipher = VigenereCipher::new("abyz");
        assert_eq!("ACAC", cipher.encrypt("ABCD"));

        let cipher = VigenereCipher::new("ABYZ");
        assert_eq!("ACAC", cipher.encrypt("ABCD"));
    }

    #[test]
    fn test_key_wraps_around() {
        let cipher = VigenereCipher::new("a");
        assert_eq!("abcdefgh", cipher.encrypt("abcdefgh"));

        let cipher = VigenereCipher::new("az");
        assert_eq!("aacceegg", cipher.encrypt("abcdefgh"));
    }

}

#[cfg(test)]
mod decrypt_tests {

    use text_cipher::TextCipher;
    use vigenere::VigenereCipher;

    #[test]
    fn test_decrypt_lowercase() {
        let cipher = VigenereCipher::new("aaaa");
        assert_eq!("abyz", cipher.decrypt("abyz"));

        let cipher = VigenereCipher::new("AAAA");
        assert_eq!("abyz", cipher.decrypt("abyz"));

        let cipher = VigenereCipher::new("abcd");
        assert_eq!("abyz", cipher.decrypt("acac"));

        let cipher = VigenereCipher::new("ABCD");
        assert_eq!("abyz", cipher.decrypt("acac"));

        let cipher = VigenereCipher::new("abyz");
        assert_eq!("aaaa", cipher.decrypt("abyz"));

        let cipher = VigenereCipher::new("ABYZ");
        assert_eq!("aaaa", cipher.decrypt("abyz"));

        let cipher = VigenereCipher::new("abyz");
        assert_eq!("abcd", cipher.decrypt("acac"));

        let cipher = VigenereCipher::new("ABYZ");
        assert_eq!("abcd", cipher.decrypt("acac"));
    }

    #[test]
    fn test_decrypt_uppercase() {
        let cipher = VigenereCipher::new("aaaa");
        assert_eq!("ABYZ", cipher.decrypt("ABYZ"));

        let cipher = VigenereCipher::new("AAAA");
        assert_eq!("ABYZ", cipher.decrypt("ABYZ"));

        let cipher = VigenereCipher::new("abcd");
        assert_eq!("ABYZ", cipher.decrypt("ACAC"));

        let cipher = VigenereCipher::new("ABCD");
        assert_eq!("ABYZ", cipher.decrypt("ACAC"));

        let cipher = VigenereCipher::new("abyz");
        assert_eq!("AAAA", cipher.decrypt("ABYZ"));

        let cipher = VigenereCipher::new("ABYZ");
        assert_eq!("AAAA", cipher.decrypt("ABYZ"));

        let cipher = VigenereCipher::new("abyz");
        assert_eq!("ABCD", cipher.decrypt("ACAC"));

        let cipher = VigenereCipher::new("ABYZ");
        assert_eq!("ABCD", cipher.decrypt("ACAC"));
    }

    #[test]
    fn test_key_wraps_around() {
        let cipher = VigenereCipher::new("a");
        assert_eq!("abcdefgh", cipher.decrypt("abcdefgh"));

        let cipher = VigenereCipher::new("az");
        assert_eq!("abcdefgh", cipher.decrypt("aacceegg"));
    }

}
