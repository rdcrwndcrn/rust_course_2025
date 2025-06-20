use std::collections::BTreeMap;
use std::io;

// finds index of char after the first whitespace
pub fn slice_to_words(sentence: &str) -> Vec<&str> {
    let mut words = Vec::new();
    for w in sentence.split_whitespace(){
        words.push(w);
    }
    words
}

pub fn length(sentence: &str) -> usize {
    sentence.len()
}

pub fn count_all_nouns(sentence: &str) -> usize {
    sentence.to_ascii_lowercase()
        .chars()
        .filter(|char| match char {
            'a' | 'e' | 'i' | 'o' | 'u' | 'ä' | 'ö' | 'ü' => true,
            _ => false
        })
        .count()
}

pub fn count_nouns(sentence: &str) -> Vec<(char, u32)> {
    let vowels = ['a', 'ä', 'e', 'i', 'o', 'ö', 'u', 'ü'];
    let mut count: BTreeMap<char, u32> =
        vowels.iter()
            .map(|&v| (v, 0))
            .collect();

    sentence
        .to_ascii_lowercase()
        .chars()
        .filter(|c| vowels.contains(c))
        .for_each(|c| *count.get_mut(&c).unwrap() += 1);

    count.into_iter().collect()
}

pub fn get_user_input() -> String {
    println!("Enter the text: ");
    let mut buffer = String::new();
    let _ = io::stdin()
        .read_line(&mut buffer)
        .expect("failed to read from stdin");
    buffer.trim().to_string()
}