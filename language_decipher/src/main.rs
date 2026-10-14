use std::io;
use std::collections::HashMap;

fn main() {
    let eng_lines = read_lines("english.txt");
    let foreign_lines = read_lines("foreign.txt");

    let words: Vec<Word> = foreign_lines
        .iter()
        .map(|line| Word {morphemes: morpheme_split(line)})
        .collect();
    
    let phrases: Vec<Phrase> = eng_lines
        .iter()
        .map(|line| Phrase {features: morpheme_split(line)})
        .collect();

    println!("{:?}", words);
    println!("{:?}", phrases);
    println!("{} {}", words.len(), phrases.len());
}


fn morpheme_split(word: &str) -> Vec<String> {
    word.trim()
    .split('-')
    .map(|s| s.trim().to_string())
    .filter(|s| !s.is_empty())
    .collect()
}

#[derive(Debug)]
struct Word {morphemes: Vec<String>}
#[derive(Debug)]
struct Phrase {features: Vec<String>}

fn read_lines(path: &str) -> Vec<String> {
    let contents = std::fs::read_to_string(path).expect("failed to read file");
    contents
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn count_morphemes(words: &[Word]) -> HashMap<String, usize> {
    let mut counts: HashMap<String usize> = HashMap::new();
    for c in ["i", "no", "u", "la", "nge", "leli", "allkü", "imi", "kim"]{
        *counts.entry(c.to_string()).or_insert(0) += 1;
    }

}

fn count_features(phrase: &[Phrase]) -> HashMap<String, usize> {
    let mut counts: HashMap<String usize> = HashMap::new();
    for c in ["eat", "neg", "imp", "reflex", "hear", "you", "know"]{
        *counts.entry(c.to_string()).or_insert(0) += 1;
    }

}
