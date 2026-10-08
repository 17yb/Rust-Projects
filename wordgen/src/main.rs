use std::io;
use rand::prelude::*;

fn read_number(prompt: &str) -> u32 {
    loop {
        println!("{prompt}");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read number");

        match input.trim().parse() {
            Ok(n) => return n,
            Err(_) => println!("Please enter a whole number"),
        }
    }
}

fn read_list(name: &str) -> Vec<String> {
    loop {
        println!("Input {name}, seperated by commas:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let list: Vec<String> = input
            .trim()
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        
        if list.is_empty() {
            println!("Please enter {name} again");
            continue;
        }

        println!("Thank you");
        return list;
    }
}

fn main(){    
    let mut rng = rand::rng();

    let consonants = read_list("consonants");
    let vowels = read_list("vowels");

    println!("Do you want finals? Type y or n");

    let mut final_q = String::new();
    io::stdin().read_line(&mut final_q).expect("Failed to read line");

    let finals = if final_q.trim() == "y" {
        read_list("finals")
    } else {
        vec![]
    };

    let syllable_structures = read_list("syllable structures");

    let word_count = read_number("Enter the number of words");

    println!("--------");

    let mut word = String::new();

    for _ in 0..word_count {
        let syllables = rng.random_range(2..5);

        for _ in 0..syllables {
            let syllable = make_syllable(&syllable_structures, &consonants, &vowels, &finals, &mut rng);
            word.push_str(&syllable);
        }
        word.push_str("\n");
    }
    println!("{}", word);
}   


fn make_syllable(syllable_structures: &[String], consonants: &[String], vowels: &[String], finals: &[String], rng: &mut impl Rng) -> String {
    let mut syllable_structure = String::new();
    let mut text = String::new();

    syllable_structure.push_str(syllable_structures.choose(rng).expect("Syllable structures was empty"));

    for character in syllable_structure.chars() {
        if character == 'C' {
            text.push_str(consonants.choose(rng).expect("Failed to read syllable"));
        } else if character == 'V' {
            text.push_str(vowels.choose(rng).expect("Failed to read syllable"));
        } else if character == 'F' {
            if let Some(f) = finals.choose(rng) {
                text.push_str(f);
            }
        }
    }
    text
}
