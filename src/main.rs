// #[macro_use] extern crate rocket;

use rayon::prelude::*;
use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};
use rocket::tokio::task::{spawn_blocking, JoinError};
use rocket::{launch, routes, get, post, tokio};
use sorted_vec::SortedSet;
use std::time::Instant;
use std::{
    cmp::Reverse,
    collections::HashMap,
    default,
    error::Error,
    io::{self, Stdin},
    num::ParseIntError,
};


#[derive(Serialize, Deserialize, Debug)]
struct Input{
    vecs: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ReturnThing{
    wow: Option<Vec<String>>,
}

#[post("/", data="<input>")]
async fn index(input: Json<Input>) -> std::io::Result<Json<ReturnThing>> {
    let a = spawn_blocking(|| 
        {
            let mut a = vec![];
            for i in 0..100000 {
                a.push(f32::sqrt(i as f32));
            }
        
            return a.into_iter().sum::<f32>();
        }
    ).await?;

    if input.vecs.len() == 1 {
        return Ok(Json(ReturnThing {wow: None}));
    } else {
        return Ok(Json(ReturnThing {wow: Some(vec![String::from("hello"), a.to_string()])}));
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}


// fn main() {
    // for y in (1..7) { for x in (0..y) {
    //         print!("*");
    //     }
    //     println!();
    //     // println!("{}", y);
    // }


// fn main() -> Result<(), Box<dyn Error>> {
//     // Naive benchmark
//     let dict_string = std::fs::read_to_string("wordlist.txt")?;
//     let word_list: Vec<&str> = dict_string.lines().collect();

//     let mut guesses = vec![];
//     let now = Instant::now();
//     for _ in 0..1 {
//         let mut hm = Hangman::with_words(&word_list, 5_usize);
//         guesses.push(hm.find_best_guess());
//         let _ = hm.add_clue(Clue::new('s', vec![]));
//         guesses.push(hm.find_best_guess());
//         let _ = hm.add_clue(Clue::new('t', vec![]));
//         guesses.push(hm.find_best_guess());
//         let _ = hm.add_clue(Clue::new('a', vec![1, 3]));
//         guesses.push(hm.find_best_guess());
//         let _ = hm.add_clue(Clue::new('u', vec![]));
//         guesses.push(hm.find_best_guess());
//         let _ = hm.add_clue(Clue::new('b', vec![0]));
//         guesses.push(hm.find_best_guess());
//     }

//     println!("{guesses:?}, {:?}", now.elapsed());

//     let mut uin = std::io::stdin();
//     println!("Hur långt är ditt ord?");
//     let word_len = read_number(&mut uin)?;

//     let dict_string = std::fs::read_to_string("wordlist.txt")?;
//     let word_list: Vec<&str> = dict_string.lines().collect();
//     let mut hm = Hangman::with_words(&word_list, word_len as usize);

//     loop {
//         let guess = hm.find_best_guess();

//         if let Some(guess) = guess {
//             println!("Innehåller ditt ord bosktaven \"{}\"? (y/n)", guess);
//             let word_has_letter = read_bool(&mut uin)?;
//             let positions = if !word_has_letter {
//                 vec![]
//             } else {
//                 println!("På vilka positioner? (0,1, 2 ...)");
//                 read_nums(&mut uin)
//                     .unwrap()
//                     .into_iter()
//                     .map(|v| v as usize)
//                     .collect()
//             };
//             hm.add_clue(Clue::new(guess, positions)).unwrap();
//         } else {
//             todo!("Handle this later");
//         }
//         dbg!(&hm.words_remaining);
//     }
// }

fn read_bool(io: &mut Stdin) -> io::Result<bool> {
    let mut input = String::new();
    io.read_line(&mut input)?;

    let char = input
        .chars()
        .next()
        .ok_or(io::Error::from(io::ErrorKind::Other))?;

    Ok(char == 'y')
}

fn read_number(io: &mut Stdin) -> io::Result<i32> {
    Ok(read_n_nums(io, 1)?[0])
}

fn read_n_nums(io: &mut Stdin, n: usize) -> io::Result<Vec<i32>> {
    let nums = read_nums(io)?;
    if nums.len() != n {
        return Err(std::io::Error::from(std::io::ErrorKind::Other));
    }

    Ok(nums)
}

fn read_nums(io: &mut Stdin) -> io::Result<Vec<i32>> {
    loop {
        let mut input = String::new();
        io.read_line(&mut input)?;

        input.pop();
        input.pop();

        let res: Option<Vec<_>> = input.split(' ').map(|v| v.parse().ok()).collect();
        if let Some(nums) = res {
            return Ok(nums);
        } else {
            println!("Invalid input. Try again");
        }
    }
}

struct Hangman {
    guessed: SortedSet<char>,
    // guesses: Guesses,
    words_remaining: Vec<String>,
    word_to_guess: Vec<Option<char>>,
}

struct Clue {
    letter: char,
    at_positions: Vec<usize>,
}

impl Clue {
    pub fn new(letter: char, at_positions: Vec<usize>) -> Self {
        Self {
            letter: letter.to_ascii_lowercase(),
            at_positions,
        }
    }
}

#[derive(Default)]
struct Guesses {
    guesses: Vec<Clue>,
}

impl Guesses {
    pub fn is_guessed(&self, letter: char) -> bool {
        self.get_guess_idx(letter).is_some()
    }

    pub fn get_guess_idx(&self, letter: char) -> Option<usize> {
        self.find_guess(letter).ok()
    }

    #[inline(always)]
    fn find_guess(&self, letter: char) -> Result<usize, usize> {
        self.guesses
            .binary_search_by_key(&letter, |guess| guess.letter)
    }

    pub fn insert(&mut self, mut guess: Clue) -> Option<Clue> {
        match self.find_guess(guess.letter) {
            Ok(i) => {
                std::mem::swap(&mut self.guesses[i], &mut guess);
                Some(guess)
            }
            Err(i) => {
                self.guesses.insert(i, guess);
                None
            }
        }
    }
}

#[derive(Debug)]
enum HangmanError {
    LetterAlreadyGuessed,
    PositionAlreadyOccupied,
}

impl Hangman {
    pub fn with_words(words: &Vec<&str>, word_len: usize) -> Self {
        let words_remaining = words
            .par_iter()
            .filter(|s| s.chars().count() == word_len)
            .map(|s| s.trim().to_ascii_lowercase())
            .collect();

        Hangman {
            words_remaining,
            guessed: SortedSet::new(),
            word_to_guess: (0..word_len).map(|_| None).collect(),
        }
    }

    pub fn find_best_guess(&self) -> Option<char> {
        let mut occurrences = self
            .count_letter_occurrences()
            .into_iter()
            .collect::<Vec<_>>();

        occurrences.sort_by_key(|(_, n)| Reverse(*n));
        let best_guess = occurrences
            .iter()
            .skip_while(|(letter, _)| self.guessed.binary_search(letter).is_ok())
            // .skip_while(|(letter, _)| self.guesses.is_guessed(*letter))
            .next()
            .map(|v| v.0);

        best_guess
    }

    pub fn add_clue(&mut self, clue: Clue) -> Result<(), HangmanError> {
        self.add_guessed(clue.letter)?;

        self.word_to_guess
            .iter_mut()
            .enumerate()
            .try_for_each(|(idx, letter)| {
                if !clue.at_positions.contains(&idx) {
                    return Ok(());
                };

                if letter.is_none() {
                    *letter = Some(clue.letter);
                    return Ok(());
                }
                // if letter.is_some() && idxs.contains(idx) { return Err(())};

                Err(())
            })
            .map_err(|_| HangmanError::PositionAlreadyOccupied)?;

        // Single threaded
        self.words_remaining.retain(|word| {
            word.chars()
                .enumerate()
                .find(|(idx, letter)| {
                    let is_same = *letter == clue.letter;
                    let should_be_same = clue.at_positions.contains(idx);

                    return is_same != should_be_same;
                })
                .is_none()
        });

        // Multithreaded approach, approx same speed
        // let mut tmp = vec![];
        // std::mem::swap(&mut tmp, &mut self.words_remaining);
        // self.words_remaining = tmp
        //     .into_par_iter()
        //     .filter(|word| {
        //         word.chars()
        //             .enumerate()
        //             .find(|(idx, letter)| {
        //                 let is_same = *letter == clue.letter;
        //                 let should_be_same = clue.at_positions.contains(idx);

        //                 return is_same != should_be_same;
        //             })
        //             .is_none()
        //     })
        //     .collect();

        Ok(())
    }

    fn add_guessed(&mut self, letter: char) -> Result<(), HangmanError> {
        if self.is_guessed(letter) {
            return Err(HangmanError::LetterAlreadyGuessed);
        }

        self.guessed.insert(letter);
        Ok(())
    }

    #[inline(always)]
    fn is_guessed(&self, letter: char) -> bool {
        self.guessed.binary_search(&letter).is_ok()
    }

    /// Count the occurrence of all letters in words remaining.
    /// Note that that each letter is only counted once per word.
    /// Eg. The words "ada" and "ale" both increace the occurrence of 'a' by 1.
    fn count_letter_occurrences(&self) -> HashMap<char, i32> {
        self.words_remaining
            .par_iter()
            .map(|word| {
                let mut charset = SortedSet::with_capacity(16);
                word.chars().for_each(|v| {
                    charset.insert(v);
                });
                charset
            })
            .fold(
                || HashMap::new(),
                |mut map, charset| {
                    charset.iter().for_each(|c| {
                        map.entry(*c).and_modify(|v| *v += 1).or_insert(1);
                    });
                    map
                },
            )
            .reduce(
                || HashMap::new(),
                |mut a, b| {
                    b.iter().for_each(|(k, v)| {
                        a.entry(*k).and_modify(|a_v| *a_v += v).or_insert(*v);
                    });
                    // a.extend(b);
                    a
                },
            )

        // Single threaded approach
        // Approx 2.3 times slower on i5 with 6 cores
        // self.words_remaining
        //     .iter()
        //     .map(|word| {
        //         // TODO Compare with performance of BinaryHeap and linear searched vector for fun.
        //         let mut charset = SortedSet::with_capacity(16);
        //         word.chars().for_each(|v| {
        //             charset.insert(v);
        //         });
        //         charset
        //     })
        //     .fold(HashMap::new(), |mut map, charset| {
        //         charset.iter().for_each(|c| {
        //             map.entry(*c).and_modify(|v| *v += 1).or_insert(1);
        //         });

        //         map
        //     })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_filter_with_positional_constraint() {
        {
            let words = vec!["abcd", "aacd", "bbcd"];
            let mut hm = Hangman::with_words(&words, 4);
            assert!(hm.add_clue(Clue::new('a', vec!(0))).is_ok());

            assert_eq!(hm.words_remaining, vec!("abcd"));
        }

        {
            let words = vec!["abca", "aaca", "aeea", "bbca", "bbcd"];
            let mut hm = Hangman::with_words(&words, 4);
            assert!(hm.add_clue(Clue::new('a', vec!(0, 3))).is_ok());

            assert_eq!(hm.words_remaining, vec!("abca", "aeea"));
        }
    }

    #[test]
    fn should_error_when_position_is_occupied() {
        {
            let words = vec!["abcd", "efgh", "ijkl"];
            let mut hm = Hangman::with_words(&words, 4);
            let _ = hm.add_clue(Clue::new('a', vec![0, 1]));
            assert!(matches!(
                hm.add_clue(Clue::new('b', vec![1, 2])),
                Err(HangmanError::PositionAlreadyOccupied)
            ))
        }
    }

    #[test]
    fn can_filter_with_not_in_word_clue() {
        {
            let words = vec!["asdf", "sdfg", "fdsa", "fdss"];
            let mut hm = Hangman::with_words(&words, 4);
            assert!(hm.add_clue(Clue::new('a', vec!())).is_ok());

            assert_eq!(hm.words_remaining, vec!("sdfg", "fdss"));
        }
    }

    #[test]
    fn returns_err_when_it_recieves_two_clues_for_same_letter() {
        {
            let words = vec!["asdf", "sdfg", "fdsa", "fdss"];
            let mut hm = Hangman::with_words(&words, 4);
            let _ = hm.add_clue(Clue::new('a', vec![])).is_ok();

            assert!(matches!(
                hm.add_clue(Clue::new('a', vec!())),
                Err(HangmanError::LetterAlreadyGuessed)
            ));
        }

        {
            let words = vec!["asdf", "sdfg", "fdsa", "fdss"];
            let mut hm = Hangman::with_words(&words, 4);
            let _ = hm.add_clue(Clue::new('a', vec![])).is_ok();

            assert!(matches!(
                hm.add_clue(Clue::new('a', vec!(0, 1))),
                Err(HangmanError::LetterAlreadyGuessed)
            ));
        }
    }

    #[test]
    fn can_count_letter_occurrences() {
        {
            let words = vec!["aaaaa", "bbbbb", "ccccc"];
            let hm = Hangman::with_words(&words, 5);

            assert_eq!(
                hm.count_letter_occurrences(),
                HashMap::from([('a', 1), ('b', 1), ('c', 1)])
            );
        }

        {
            let words = vec!["aaaa", "bbbbb", "ccccc"];
            let hm = Hangman::with_words(&words, 5);

            assert_eq!(
                hm.count_letter_occurrences(),
                HashMap::from([('b', 1), ('c', 1)])
            );
        }

        {
            let words = vec!["aaaaa", "babbb", "ccccc"];
            let hm = Hangman::with_words(&words, 5);

            assert_eq!(
                hm.count_letter_occurrences(),
                HashMap::from([('a', 2), ('b', 1), ('c', 1)])
            );
        }

        {
            let words = vec!["ééééé", "babbb", "ccccc"];
            let hm = Hangman::with_words(&words, 5);

            assert_eq!(
                hm.count_letter_occurrences(),
                HashMap::from([('a', 1), ('b', 1), ('c', 1), ('é', 1)])
            );
        }
    }
}
