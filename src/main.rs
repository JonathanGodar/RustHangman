use std::{
    collections::{HashMap, HashSet},
    env,
    error::Error,
    fs::{self, File},
    io::{self, BufRead, BufReader},
    path::Path,
    usize, char, fmt::Debug,
};

fn main() -> Result<(), std::io::Error> {
    let file = fs::File::open("wordlist.txt")?;
    let buffer = BufReader::new(file);

    let word = String::from("hajpådajdu");

    let mut words = Vec::with_capacity(200000);
    for line in buffer.lines() {
        match line {
            Ok(word) => words.push(word.trim().to_lowercase().to_owned()),
            Err(_) => println!("hello world"),
        }
    }

    let stdin = io::stdin();

    let mut user_input = String::new();
    println!("hur många bokstäver är ditt ord?");
    stdin.read_line(&mut user_input)?;
    let char_count = user_input.trim().parse::<usize>().unwrap();

    for i in (0..words.len()).rev() {
        if words[i].chars().count() != char_count {
            words.remove(i);
        }
    }

    let mut guessed: Vec<char> = vec![];
    while let Some(best_guess) = best_guess(&words, &guessed){
        guessed.push(best_guess);

        let a = get_user_word_info(best_guess);
        remove_non_compliant(&mut words, a);
        if words.len() < 2 {
            break;
        }

        println!("{:#?}", words);
    }

    if(words.len() > 0 ){
        println!("Ditt ord var: {}", words[0]);
    } else {
        println!("Kunde inte hitta ordet!");
    }

    Ok(())
}

struct HangManGame {
    guessed: ,
}


#[derive(Debug)]
enum WordInfo {
    DoesNotContain(char),
    Contains(LetterPositions),
}

#[derive(Debug)]
struct LetterPositions{
    letter: char,
    positions: Vec<usize>,
}


use WordInfo::{DoesNotContain, Contains};


fn get_user_word_info(letter: char) -> WordInfo {
    let stdin = io::stdin();

    let mut user_input = String::new();
    println!("Finns bokstaven \"{}\" med? j/n", letter);
    stdin.read_line(&mut user_input).unwrap();

    if user_input.starts_with("n") {
        return WordInfo::DoesNotContain(letter);
    }

    let positions = get_user_letter_positions();

    WordInfo::Contains(LetterPositions {letter, positions})
}

fn get_user_letter_positions() -> Vec<usize>{
    let stdin = io::stdin();

    let mut user_input = String::new();
    println!("Vilka positoiner? 0 1 3");
    stdin.read_line(&mut user_input).unwrap();

    user_input.split_whitespace().map(|s| s.parse().unwrap()).collect()
}


fn remove_non_compliant(words: &mut Vec<String>, info: WordInfo){
    match info {
        DoesNotContain(letter) => {
            for i in (0..words.len()).rev()  {
                if words[i].contains(letter){
                    words.remove(i);
                }
            }
        }
        Contains(info) => {
            for i in (0..words.len()).rev() {
                if !word_contains_on_right_positions(&words[i], &info){
                    words.remove(i);
                }
            }
        },
    }
}

fn best_guess(words: &Vec<String>, guessed: &Vec<char>) -> Option<char>{
    let occurences = count_occurences(words);

    let mut iter = occurences.iter();
    let mut best_guess = iter.next()?;

    for i in iter{
        if guessed.contains(i.0){
            continue;
        }

        if best_guess.1 < i.1 {
            best_guess = i;
        }
    }

    if guessed.contains(best_guess.0){
        return None;
    }

    Some(*best_guess.0)
}

fn count_occurences(words: &Vec<String>) -> HashMap<char, i32>{
    let mut letter_freq = HashMap::new();
    for word in words {
        let chars = word.chars().collect::<HashSet<_>>().into_iter();

        for letter in chars {
            *letter_freq.entry(letter).or_insert(0) += 1;
        }
    }
    letter_freq
}


fn word_contains_on_right_positions(word: &str, letterPos: &LetterPositions) -> bool{
    let chars = word.chars();
    for thing in word.chars().enumerate() {
        if letterPos.positions.contains(&thing.0.into()) {
            if letterPos.letter != thing.1{
                return false
            }
        } else {
            if letterPos.letter == thing.1{
                return false;
            }
        }
    }
    true
}



// fn count_graphemes(&str thing) -> usize{
//     thing.graphemes(true).count()
// }

// fn read_lines(fileName: Path) -> io::Result<io::Lines<io::BufReader<File>>>{
//     let file = fs::File::open(fileName)?;
//     Ok(io::BufReader::new(file))
