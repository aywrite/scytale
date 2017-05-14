use caesar_shift::lower_caesar_shift;
use caesar_shift::upper_caesar_shift;

/// Returns vigenere encrpyted version of the input string
///
/// Only latin alphabet characters will be modified, any other characters, numbers, punctuation etc.
/// will be left as is. Case is also preserved.  A vigenere cipher takes each character in the input
/// string and adds the character with the same index from the key string to its alphanumeric value
/// (mod 26) and converts it back to a new character. If the end of the key is reached before the
/// end of the message then the cipher starts again from the begining of the key. "a" is treated as
/// a shift of 0 and the case of the key is ignored.
///
/// **Parameters**
/// - text (String): The text to be encrypted
/// - key (String): The value to be added to each character
///
/// # Examples
///
/// ```
/// use scytale::vigenere::encrypt;
/// use scytale::vigenere::decrypt;
///
/// ```

pub fn encrypt(text: &str, key: &str) -> String {
    let mut result = String::new();
    for (c, k) in text.chars().zip(key.to_lowercase().chars().cycle()) {
        let shift = (k as u8) - 97;
        result.push(shift_char(c, shift));
    }
    result
}


/// Returns vigenere decrypted version of the input string
///
/// Only latin alphabet characters will be modified, any other characters, numbers, punctuation
/// etc. will be left as is. Case is also preserved.  A vigenere cipher takes each character in the
/// input string and adds the character with the same index from the key string to its alphanumeric
/// value (mod 26) and converts it back to a new character. If the end of the key is reached before
/// the end of the message then the cipher starts again from the begining of the key. "a" is
/// treated as a shift of 0 and the case of the key is ignored.
///
/// **Parameters**
/// - text (String): The text to be encrypted
/// - key (String): The value to be added to each character
///
/// # Examples
///
/// ```
/// use scytale::vigenere::encrypt;
/// use scytale::vigenere::decrypt;
///
/// ```

pub fn decrypt(text: &str, key: &str) -> String {
    let mut result = String::new();
    for (c, k) in text.chars().zip(key.to_lowercase().chars().cycle()) {
        let shift = (26 - (k as u8 - 97)) % 26;
        result.push(shift_char(c, shift));
    }
    result
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

    use vigenere::encrypt;

    #[test]
    fn test_encrypt_lowercase() {
        assert_eq!("abyz", encrypt("abyz", "aaaa"));
        assert_eq!("abyz", encrypt("abyz", "AAAA"));
        assert_eq!("acac", encrypt("abyz", "abcd"));
        assert_eq!("acac", encrypt("abyz", "ABCD"));
        assert_eq!("abyz", encrypt("aaaa", "abyz"));
        assert_eq!("abyz", encrypt("aaaa", "ABYZ"));
        assert_eq!("acac", encrypt("abcd", "abyz"));
        assert_eq!("acac", encrypt("abcd", "ABYZ"));
    }

    #[test]
    fn test_encrypt_uppercase() {
        assert_eq!("ABYZ", encrypt("ABYZ", "aaaa"));
        assert_eq!("ABYZ", encrypt("ABYZ", "AAAA"));
        assert_eq!("ACAC", encrypt("ABYZ", "abcd"));
        assert_eq!("ACAC", encrypt("ABYZ", "ABCD"));
        assert_eq!("ABYZ", encrypt("AAAA", "abyz"));
        assert_eq!("ABYZ", encrypt("AAAA", "ABYZ"));
        assert_eq!("ACAC", encrypt("ABCD", "abyz"));
        assert_eq!("ACAC", encrypt("ABCD", "ABYZ"));
    }

    #[test]
    fn test_key_wraps_around() {
        assert_eq!("abcdefgh", encrypt("abcdefgh", "a"));
        assert_eq!("aacceegg", encrypt("abcdefgh", "az"));
    }

}

#[cfg(test)]
mod decrypt_tests {

    use vigenere::decrypt;

    #[test]
    fn test_decrypt_lowercase() {
        assert_eq!("abyz", decrypt("abyz", "aaaa"));
        assert_eq!("abyz", decrypt("abyz", "AAAA"));
        assert_eq!("abyz", decrypt("acac", "abcd"));
        assert_eq!("abyz", decrypt("acac", "ABCD"));
        assert_eq!("aaaa", decrypt("abyz", "abyz"));
        assert_eq!("aaaa", decrypt("abyz", "ABYZ"));
        assert_eq!("abcd", decrypt("acac", "abyz"));
        assert_eq!("abcd", decrypt("acac", "ABYZ"));
    }

    #[test]
    fn test_decrypt_uppercase() {
        assert_eq!("ABYZ", decrypt("ABYZ", "aaaa"));
        assert_eq!("ABYZ", decrypt("ABYZ", "AAAA"));
        assert_eq!("ABYZ", decrypt("ACAC", "abcd"));
        assert_eq!("ABYZ", decrypt("ACAC", "ABCD"));
        assert_eq!("AAAA", decrypt("ABYZ", "abyz"));
        assert_eq!("AAAA", decrypt("ABYZ", "ABYZ"));
        assert_eq!("ABCD", decrypt("ACAC", "abyz"));
        assert_eq!("ABCD", decrypt("ACAC", "ABYZ"));
    }

    #[test]
    fn test_key_wraps_around() {
        assert_eq!("abcdefgh", decrypt("abcdefgh", "a"));
        assert_eq!("abcdefgh", decrypt("aacceegg", "az"));
    }

}
