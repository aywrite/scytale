use ngrams::Ngram;
use std::collections::HashMap;

pub fn count_chars(text: &str) -> String {
    let mut frequency: HashMap<char, u32> = HashMap::new();
    for c in text.chars() {
        *frequency.entry(c).or_insert(0) += 1;
    }
    let mut sorted = frequency.iter().collect::<Vec<(&char, &u32)>>();
    sorted.sort_by(|&a, &b| a.0.cmp(b.0));

    sorted
        .iter()
        .map(|(k, v)| format!("{},{}\n", k, v))
        .collect()
}

pub fn count_words(text: &str) -> String {
    let mut frequency: HashMap<String, u32> = HashMap::new();
    for word in text.split(" ") {
        *frequency.entry(word.to_owned()).or_insert(0) += 1;
    }
    let mut sorted = frequency.iter().collect::<Vec<(&String, &u32)>>();
    sorted.sort_by(|&a, &b| a.1.cmp(b.1));

    sorted
        .iter()
        .map(|(k, v)| format!("{},{}\n", k, v))
        .collect()
}

pub fn count_ngrams(text: &str, length: usize) -> String {
    let mut frequency: HashMap<String, u32> = HashMap::new();
    for ng in text.chars().ngrams(length).pad().collect::<Vec<_>>() {
        *frequency.entry(ng.iter().collect()).or_insert(0) += 1;
    }
    let mut sorted = frequency.iter().collect::<Vec<(&String, &u32)>>();
    sorted.sort_by(|&a, &b| a.1.cmp(b.1));

    sorted
        .iter()
        .map(|(k, v)| format!("{},{}\n", k, v))
        .collect()
}
