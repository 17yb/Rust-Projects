use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let (words, phrases) = load("foreign.txt", "english.txt");

    let pairing: Vec<usize> = vec![0, 1, 2, 3, 7, 5, 4, 6, 8];

    let imi_features = common_features(&words, &phrases, &pairing, "imi");
    println!("imi: {:?}", imi_features);
    let nge_features = common_features(&words, &phrases, &pairing, "nge");
    println!("nge: {:?}", nge_features);
    let leli_features = common_features(&words, &phrases, &pairing, "leli");
    println!("leli: {:?}", leli_features);

    let morpheme_counts = count_morphemes(&words);
    let feature_counts = count_features(&phrases);

    print_counts("Morphemes", &morpheme_counts);
    print_counts("Features", &feature_counts);

    let roots: HashSet<&String> = words.iter().filter_map(|w| w.morphemes.first()).collect();

    let mut names: Vec<&String> = morpheme_counts.keys().collect();
    names.sort();

    for i in 0..names.len() {
        for j in (i + 1)..names.len() {
            if roots.contains(names[i]) || roots.contains(names[j]) {
                continue;
            }
            if !co_occur(&words, names[i], names[j]) {
                println!("{} and {} do not appear together", names[i], names[j]);
            }
        }
    }
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
    let mut counts: HashMap<String, usize> = HashMap::new();
    for word in words {
        for m in &word.morphemes {
            *counts.entry(m.to_string()).or_insert(0) += 1;
        }
    }
    counts
}

fn count_features(phrases: &[Phrase]) -> HashMap<String, usize> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for phrase in phrases {
        for f in &phrase.features {
            *counts.entry(f.to_string()).or_insert(0) += 1;
        }
    }
    counts
}

fn print_counts(title: &str, counts: &HashMap<String, usize> ) {
    println!("{}:", title);

    let mut sorted: Vec<_> = counts.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));    

    for (name, count) in sorted { 
        println!("{}: {}", name, count)
    }
}

fn co_occur(words: &[Word], a: &str, b: &str) -> bool {
    words.iter().any(|word| has(word, a) && has(word, b))
}

fn has(word: &Word, m: &str) -> bool {
    word.morphemes.iter().any(|x| x == m)
}

fn common_features(words: &[Word], phrases: &[Phrase], pairing: &[usize], morpheme: &str) -> HashSet<String> {
    let mut result: Option<HashSet<String>> = None;
    for (i, w) in words.iter().enumerate() {
        if has(w, morpheme) {
            let phrase = &phrases[pairing[i]];
            if let Some(set) = &mut result {
                set.retain(|f|phrase.features.contains(f)); 
            } else {   
                result = Some(phrase.features.iter().cloned().collect());
            }
        }
    }
    result.unwrap_or_default()
}

fn load(foreign_path: &str, english_path: &str) -> (Vec<Word>, Vec<Phrase>) {
    let eng_lines = read_lines(english_path);
    let foreign_lines = read_lines(foreign_path);

    let words: Vec<Word> = foreign_lines
        .iter()
        .map(|line| Word {morphemes: morpheme_split(line)})
        .collect();

    let phrases: Vec<Phrase> = eng_lines
        .iter()
        .map(|line| Phrase {features: morpheme_split(line)})
        .collect();
    
    (words, phrases)
}

fn is_consistent(words: &[Word], phrases: &[Phrase], pairing: &[usize])-> bool{
    let mut candidates: HashMap<String, HashSet<String>> = HashMap::new();
    for word in words {
        for m in &word.morphemes {
            if !candidates.contains_key(m) {
                let set = common_features(words, phrases, pairing, m);
                if set.is_empty() {
                    return false;
                }
                candidates.insert(m.clone(), set);
            }
        }
    }
    for (i, words) in words.iter().enumerate(){
        let phrase = &phrases[pairing[i]];
        let mut union: HashSet<String> = HashSet::new();

        for m in &words.morphemes {
            union.extend(candidates[m].iter().cloned());
        }
        for f in &phrase.features {
            if !union.contains(f) {
                return false;
            }
        }
    }
    true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_pairing_is_consistent(){
        let (words, phrases) = load("foreign.txt", "english.txt");
        let mut pairing: Vec<usize> = vec![0, 1, 2, 3, 7, 5, 4, 6, 8];

        assert!(is_consistent(&words, &phrases, &pairing));
    }

    #[test]
    fn swapped_pairing_is_consistent() {
        let (words, phrases) = load("foreign.txt", "english.txt");
        let mut pairing: Vec<usize> = vec![0, 1, 2, 3, 7, 5, 4, 6, 8];
        pairing.swap(0,1);
        assert!(!is_consistent(&words, &phrases, &pairing));
    }

}


