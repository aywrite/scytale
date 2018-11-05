pub trait TextCipher {
    fn encrypt(&self, text: &str) -> String;
    fn decrypt(&self, text: &str) -> String;
}
