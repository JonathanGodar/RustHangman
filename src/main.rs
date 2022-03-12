use std::{
    char,
    collections::{HashMap, HashSet},
    env,
    error::Error,
    fmt::Debug,
    fs::{self, File},
    io::{self, BufRead, BufReader},
    path::Path,
    string, usize,
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
    while let Some(best_guess) = best_guess(&words, &guessed) {
        guessed.push(best_guess);

        let a = get_user_word_info(best_guess);
        remove_non_compliant(&mut words, a);
        if words.len() < 2 {
            break;
        }

        println!("{:#?}", words);
    }

    if words.len() > 0 {
        println!("Ditt ord var: {}", words[0]);
    } else {
        println!("Kunde inte hitta ordet!");
    }

    Ok(())
}

struct HangManGame {
    guessed: SortedVec<char>,
    words: Vec<String>,
    word_length: usize,

    letter_constraints: HashMap<char, SortedVec<usize>>,
    not_allowed_letters: HashSet<char>,
}

impl HangManGame {
    fn new(word_length: usize, dictionary: Vec<String>) -> HangManGame {
        let dictionary = dictionary
            .into_iter()
            .filter(|word| word.len() == word_length)
            .collect();

        let game = HangManGame {
            word_length,
            words: dictionary,
            guessed: SortedVec::new(),
            letter_constraints: HashMap::new(),
            not_allowed_letters: HashSet::new(),
        };

        game
    }

    fn add_constraint(&mut self, constraint: WordConstraint) {
        match constraint {
            DoesNotContain(letter) => {
                self.apply_does_not_contain_constraint(letter);
            }
            Contains(letter_info) => {
                self.apply_letter_constraint(letter_info);
            }
        }
    }

    fn apply_does_not_contain_constraint(&mut self, letter: char) -> bool {
        if self.letter_constraints.contains_key(&letter) {
            return false;
        }
        self.not_allowed_letters.insert(letter);

        self.words.retain(|word| !word.contains(letter));
        true
    }

    fn apply_letter_constraint(&mut self, constraint: LetterPositions) -> bool {
        let remaining: Vec<String> = Vec::with_capacity(self.words.len());

        self.words
            .retain(|word| word_contains_on_right_positions(word, &constraint));

        self.letter_constraints
            .insert(constraint.letter, constraint.positions);
        true
    }

    fn word_matches_constraint(word: &String, constraint: LetterPositions) -> bool {
        let mut ptr = 0;
        let letter = constraint.letter;
        let positions = constraint.positions;

        for item in word.chars().enumerate() {
            if item.0 == positions[ptr] {
                if item.1 != letter {
                    return false;
                }
                ptr = std::cmp::max(ptr + 1, positions.len() - 1);
            } else if item.1 == letter {
                return false;
            }
        }
        return true;
    }
}

// fn apply_queued_constraints(&mut self) {
//     let mut not_allowed = HashSet::new();
//     let mut letter_constraints = HashMap::new();
//
//     for constraint in &self.queued_constraints {
//         match constraint {
//             DoesNotContain(letter) => not_allowed.insert(letter),
//             Contains(letter_constraint) => {
//                 letter_constraints.insert(letter_constraint.letter, letter_constraint.positions);
//             },
//         }
//     }
//
//     self.queued_constraints = vec![];
//
//     self.words = self.words.into_iter().filter(|word| {
//
//     })
//
// }

#[derive(Debug)]
enum WordConstraint {
    DoesNotContain(char),
    Contains(LetterPositions),
}

#[derive(Debug)]
struct LetterPositions {
    letter: char,
    positions: SortedVec<usize>,
}

use sorted_vec::SortedVec;
use WordConstraint::{Contains, DoesNotContain};

fn get_user_word_info(letter: char) -> WordConstraint {
    let stdin = io::stdin();

    let mut user_input = String::new();
    println!("Finns bokstaven \"{}\" med? j/n", letter);
    stdin.read_line(&mut user_input).unwrap();

    if user_input.starts_with("n") {
        return WordConstraint::DoesNotContain(letter);
    }

    let positions = get_user_letter_positions();

    WordConstraint::Contains(LetterPositions { letter, positions })
}

fn get_user_letter_positions() -> SortedVec<usize> {
    let stdin = io::stdin();

    let mut user_input = String::new();
    println!("Vilka positoiner? 0 1 3");
    stdin.read_line(&mut user_input).unwrap();

    let mut return_val: SortedVec<usize> = SortedVec::new();

    let a: Vec<usize>= user_input
        .split_whitespace()
        .map(|s| s.parse().unwrap()).collect();

    for val in a {
        return_val.insert(val.to_owned());
    }

    return_val
}

fn remove_non_compliant(words: &mut Vec<String>, info: WordConstraint) {
    match info {
        DoesNotContain(letter) => {
            for i in (0..words.len()).rev() {
                if words[i].contains(letter) {
                    words.remove(i);
                }
            }
        }
        Contains(info) => {
            for i in (0..words.len()).rev() {
                if !word_contains_on_right_positions(&words[i], &info) {
                    words.remove(i);
                }
            }
        }
    }
}

fn best_guess(words: &Vec<String>, guessed: &Vec<char>) -> Option<char> {
    let occurences = count_occurences(words);

    let mut iter = occurences.iter();
    let mut best_guess = iter.next()?;

    for i in iter {
        if guessed.contains(i.0) {
            continue;
        }

        if best_guess.1 < i.1 {
            best_guess = i;
        }
    }

    if guessed.contains(best_guess.0) {
        return None;
    }

    Some(*best_guess.0)
}

fn count_occurences(words: &Vec<String>) -> HashMap<char, i32> {
    let mut letter_freq = HashMap::new();
    for word in words {
        let chars = word.chars().collect::<HashSet<_>>().into_iter();

        for letter in chars {
            *letter_freq.entry(letter).or_insert(0) += 1;
        }
    }
    letter_freq
}

fn word_contains_on_right_positions(word: &str, letter_pos: &LetterPositions) -> bool {
    let chars = word.chars();
    for thing in word.chars().enumerate() {
        if letter_pos.positions.contains(&thing.0.into()) {
            if letter_pos.letter != thing.1 {
                return false;
            }
        } else {
            if letter_pos.letter == thing.1 {
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
