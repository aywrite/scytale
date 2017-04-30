/// Returns a rot13 encrpyted version of the input string  
///
/// Only latin alphabet characters will be encrypted, any
/// other characters, numbers, punctuation etc. will be left as
/// is. Case is also preserved. rot13 takes each character
/// adds 13 to its alphanumeric value (1-26) and converts it back
/// to a new character. Since it is a shift of 13 in a length 26
/// alphabet, computing rot13 again on the output will return the
/// original input.
///
/// **Parameters**
/// - text (String): The text to be encrypted
///
/// # Examples
///
/// ```
/// use scytale::rot13::rot13;
///
/// assert_eq!("nolm", rot13("abyz"));
/// assert_eq!("NOLM", rot13("ABYZ"));
/// assert_eq!(
///     "Url! Jung gvzr vf vg?",
///     rot13("Hey! What time is it?")
/// );
/// assert_eq!("abcde", rot13(&rot13("abcde")));
/// ```

pub fn rot13(text: &str) -> String {
    text.chars().map(|c| {
        match c {
            'A'...'Z' => upper_rot13(c),
            'a'...'z' => lower_rot13(c),
            _ => c
        }
    }).collect()
}

fn upper_rot13(c: char) -> char {
   let mut num_c = c as u8;
   num_c = ((num_c - 65) + 13) % 26 + 65;
   num_c as char
}

fn lower_rot13(c: char) -> char {
   let mut num_c = c as u8;
   num_c = ((num_c - 97) + 13) % 26 + 97;
   num_c as char
}


#[cfg(test)]
mod tests {

    use rot13::rot13;

    #[test]
    fn test_rot13_with_simple_lowercase() {
        assert_eq!("nolm", rot13("abyz"));
    }

    #[test]
    fn test_rot13_with_simple_uppercase() {
        assert_eq!("NOLM", rot13("ABYZ"));
    }

    #[test]
    fn test_rot13_with_punctuation() {
        assert_eq!(
            "Url! Jung gvzr vf vg?",
            rot13("Hey! What time is it?")
        );
    }

    #[test]
    fn test_rot13_is_reveresed_by_applying_twice() {
        let word = "abcde";
        assert_eq!(word, rot13(&rot13(word)));
    }
}
