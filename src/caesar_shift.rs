/// Returns caesar shift encrpyted version of the input string
///
/// Only latin alphabet characters will be encrypted, any
/// other characters, numbers, punctuation etc. will be left as
/// is. Case is also preserved.
/// A caesar shift takes each character adds the input
/// number/key to its alphanumeric value (mod 26) and converts
/// it back to a new character.
///
/// **Parameters**
/// - text (String): The text to be encrypted
/// - shift (integer): The value to be added to each character
///

/// Performs an inverse caesar shift on the input string.
///
/// Only latin alphabet characters will be decrypted, any
/// other characters, numbers, punctuation etc. will be left as
/// is. Case is also preserved.
/// A caesar shift takes each character adds the input
/// number/key to its alphanumeric value (mod 26) and converts
/// it back to a new character.
///
/// **Parameters**
/// - text (String): The text to be decrypted
/// - shift (integer): The value to be subtracted from each character
///     this should be the same value that was used to encrypt.
///
use text_cipher::TextCipher;

#[derive(Debug, Clone)]
pub struct CaesarCipher {
    key: u32,
}

impl CaesarCipher {
    pub fn new(key: u32) -> CaesarCipher {
        CaesarCipher { key }
    }
}

impl TextCipher for CaesarCipher {
    fn encrypt(&self, text: &str) -> String {
        let shift = (self.key % 26) as u8;
        caesar_shift(text, shift)
    }

    fn decrypt(&self, text: &str) -> String {
        let shift = 26 - (self.key % 26) as u8;
        caesar_shift(text, shift)
    }
}

fn caesar_shift(text: &str, shift: u8) -> String {
    text.chars()
        .map(|c| match c {
            'A'...'Z' => upper_caesar_shift(c, shift),
            'a'...'z' => lower_caesar_shift(c, shift),
            _ => c,
        })
        .collect()
}

pub fn upper_caesar_shift(c: char, shift: u8) -> char {
    let mut num_c = c as u8;
    num_c = ((num_c - 65) + shift) % 26 + 65;
    num_c as char
}

pub fn lower_caesar_shift(c: char, shift: u8) -> char {
    let mut num_c = c as u8;
    num_c = ((num_c - 97) + shift) % 26 + 97;
    num_c as char
}

#[cfg(test)]
mod caeser_tests {

    use caesar_shift::caesar_shift;

    #[test]
    fn test_caesar_shift_lowercase() {
        assert_eq!("abyz", caesar_shift("abyz", 0));
        assert_eq!("bcza", caesar_shift("abyz", 1));
        assert_eq!("nolm", caesar_shift("abyz", 13));
        assert_eq!("zaxy", caesar_shift("abyz", 25));
        assert_eq!("abyz", caesar_shift("abyz", 26));
    }

    #[test]
    fn test_caesar_shift_uppercase() {
        assert_eq!("ABYZ", caesar_shift("ABYZ", 0));
        assert_eq!("BCZA", caesar_shift("ABYZ", 1));
        assert_eq!("NOLM", caesar_shift("ABYZ", 13));
        assert_eq!("ZAXY", caesar_shift("ABYZ", 25));
        assert_eq!("ABYZ", caesar_shift("ABYZ", 26));
    }

}

#[cfg(test)]
mod encrypt_tests {

    use caesar_shift::CaesarCipher;
    use text_cipher::TextCipher;

    #[test]
    fn test_encrypt_lowercase() {
        let cipher = CaesarCipher::new(0);
        assert_eq!("abyz", cipher.encrypt("abyz"));

        let cipher = CaesarCipher::new(1);
        assert_eq!("bcza", cipher.encrypt("abyz"));

        let cipher = CaesarCipher::new(13);
        assert_eq!("nolm", cipher.encrypt("abyz"));

        let cipher = CaesarCipher::new(25);
        assert_eq!("zaxy", cipher.encrypt("abyz"));

        let cipher = CaesarCipher::new(26);
        assert_eq!("abyz", cipher.encrypt("abyz"));
    }

    #[test]
    fn test_encrypt_uppercase() {
        let cipher = CaesarCipher::new(0);
        assert_eq!("ABYZ", cipher.encrypt("ABYZ"));

        let cipher = CaesarCipher::new(1);
        assert_eq!("BCZA", cipher.encrypt("ABYZ"));

        let cipher = CaesarCipher::new(13);
        assert_eq!("NOLM", cipher.encrypt("ABYZ"));

        let cipher = CaesarCipher::new(25);
        assert_eq!("ZAXY", cipher.encrypt("ABYZ"));

        let cipher = CaesarCipher::new(26);
        assert_eq!("ABYZ", cipher.encrypt("ABYZ"));
    }

}

#[cfg(test)]
mod decrypt_tests {

    use caesar_shift::CaesarCipher;
    use caesar_shift::TextCipher;

    #[test]
    fn test_decrypt_lowercase() {
        let cipher = CaesarCipher::new(0);
        assert_eq!("abyz", cipher.decrypt("abyz"));

        let cipher = CaesarCipher::new(1);
        assert_eq!("abyz", cipher.decrypt("bcza"));

        let cipher = CaesarCipher::new(13);
        assert_eq!("abyz", cipher.decrypt("nolm"));

        let cipher = CaesarCipher::new(25);
        assert_eq!("abyz", cipher.decrypt("zaxy"));

        let cipher = CaesarCipher::new(26);
        assert_eq!("abyz", cipher.decrypt("abyz"));
    }

    #[test]
    fn test_decrypt_uppercase() {
        let cipher = CaesarCipher::new(0);
        assert_eq!("ABYZ", cipher.decrypt("ABYZ"));

        let cipher = CaesarCipher::new(1);
        assert_eq!("ABYZ", cipher.decrypt("BCZA"));

        let cipher = CaesarCipher::new(13);
        assert_eq!("ABYZ", cipher.decrypt("NOLM"));

        let cipher = CaesarCipher::new(25);
        assert_eq!("ABYZ", cipher.decrypt("ZAXY"));

        let cipher = CaesarCipher::new(26);
        assert_eq!("ABYZ", cipher.decrypt("ABYZ"));
    }
}
