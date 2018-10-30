//! A basic implementation of a substitution cipher in Rust.
//!
//! Takes a map of plain text characters to a string of cipher text characters which can be used to
//! encrypt/decrypt an alphabetic message (ASCII only).
//!
//! # Examples
//!
//! ```
//! use scytale::substitution::SubstitutionCipher;
//! use std::collections::HashMap;
//!
//! let mut key: HashMap<char, String> = HashMap::new();
//! key.insert('a', "hey!".to_string());
//! key.insert('b', "world".to_string());
//! let cipher = &SubstitutionCipher::new(key);
//! assert_eq!("hey! world", cipher.encrypt("aaaa bbbbb"));
//! assert_eq!("aaaa bbbbb", cipher.decrypt("hey! world"));
//! ```
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SubstitutionCipher {
    key: HashMap<char, String>,
}

impl SubstitutionCipher {
    pub fn new(key: HashMap<char, String>) -> SubstitutionCipher {
        SubstitutionCipher { key }
    }

    pub fn encrypt(&self, text: &str) -> String {
        text.chars()
            .enumerate()
            .map(|(i, c)| match self.key.get(&c) {
                Some(r) => r.chars().nth(i % r.len()).unwrap_or(c),
                _ => c,
            })
            .collect()
    }

    pub fn decrypt(&self, text: &str) -> String {
        let mut map2 = HashMap::new();
        for (k, v) in self.key.clone() {
            for c in v.chars() {
                map2.insert(c, k);
            }
        }
        text.chars()
            .map(|c| match map2.get(&c) {
                Some(&r) => r,
                _ => c,
            })
            .collect()
    }
}

#[cfg(test)]
mod decrypt_tests {

    use std::collections::HashMap;
    use substitution::SubstitutionCipher;

    #[test]
    fn test_decrypt() {
        let mut key: HashMap<char, String> = HashMap::new();
        key.insert('a', "12".to_string());
        key.insert('z', "3".to_string());
        let cipher = &SubstitutionCipher::new(key);
        assert_eq!("aabz", cipher.decrypt("12b3"));
    }
}

#[cfg(test)]
mod encrypt_tests {

    use std::collections::HashMap;
    use substitution::SubstitutionCipher;

    #[test]
    fn test_encrypt() {
        let mut key: HashMap<char, String> = HashMap::new();
        key.insert('a', "12".to_string());
        key.insert('z', "3".to_string());
        let cipher = SubstitutionCipher::new(key);
        assert_eq!("12b3", cipher.encrypt("aabz"));
    }
}
