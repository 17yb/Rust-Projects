use std::io;
use rand::prelude::*;

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

    println!("--------");

    for _ in 0..5 {
        let mut word = String::new();
        let syllables = rng.random_range(2..5);

        for _ in 0..syllables {
            word.push_str(consonants.choose(&mut rng).expect("Consonants was empty"));
            word.push_str(vowels.choose(&mut rng).expect("Vowels was empty"));

            if final_q.trim() == "y" {
                let f = finals.choose(&mut rng).expect("Finals was empty");
                word.push_str(f);
            }
        }
        println!("{}", word);
    }   
}
