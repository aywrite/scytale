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
/// # Examples
///
/// ```
/// use scytale::caesar_shift::encrypt;
/// use scytale::caesar_shift::decrypt;
///
/// ```

pub fn encrypt(text: &str, shift: u32) -> String {
    let shift = (shift % 26) as u8;
    caesar_shift(text, shift)
}

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
/// # Examples
///
/// ```
/// use scytale::caesar_shift::encrypt;
/// use scytale::caesar_shift::decrypt;
///
/// ```

pub fn decrypt(text: &str, shift: u32) -> String {
    let shift = 26 - (shift % 26) as u8;
    caesar_shift(text, shift)
}

fn caesar_shift(text: &str, shift: u8) -> String {
    text.chars().map(|c| {
        match c {
            'A'...'Z' => upper_caesar_shift(c, shift),
            'a'...'z' => lower_caesar_shift(c, shift),
            _ => c
        }
    }).collect()
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

    use caesar_shift::encrypt;

    #[test]
    fn test_encrypt_lowercase() {
        assert_eq!("abyz", encrypt("abyz", 0));
        assert_eq!("bcza", encrypt("abyz", 1));
        assert_eq!("nolm", encrypt("abyz", 13));
        assert_eq!("zaxy", encrypt("abyz", 25));
        assert_eq!("abyz", encrypt("abyz", 26));
    }

    #[test]
    fn test_encrypt_uppercase() {
        assert_eq!("ABYZ", encrypt("ABYZ", 0));
        assert_eq!("BCZA", encrypt("ABYZ", 1));
        assert_eq!("NOLM", encrypt("ABYZ", 13));
        assert_eq!("ZAXY", encrypt("ABYZ", 25));
        assert_eq!("ABYZ", encrypt("ABYZ", 26));
    }

}

#[cfg(test)]
mod decrypt_tests {

    use caesar_shift::decrypt;

    #[test]
    fn test_decrypt_lowercase() {
        assert_eq!("abyz", decrypt("abyz", 0));
        assert_eq!("abyz", decrypt("bcza", 1));
        assert_eq!("abyz", decrypt("nolm", 13));
        assert_eq!("abyz", decrypt("zaxy", 25));
        assert_eq!("abyz", decrypt("abyz", 26));
    }

    #[test]
    fn test_decrypt_uppercase() {
        assert_eq!("ABYZ", decrypt("ABYZ", 0));
        assert_eq!("ABYZ", decrypt("BCZA", 1));
        assert_eq!("ABYZ", decrypt("NOLM", 13));
        assert_eq!("ABYZ", decrypt("ZAXY", 25));
        assert_eq!("ABYZ", decrypt("ABYZ", 26));
    }

}
