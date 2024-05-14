use crate::word_place::Character;
use crate::word_place::WordPlace;
use crate::Direction::{Horizontal, Vertical};
use rayon::prelude::*;

#[derive(PartialEq, Debug)]
pub(crate) enum SolveState {
    Solved(Vec<WordPlace>),
    Unsolved,
}

pub(crate) fn solve(wordlist: &Vec<impl AsRef<str>>, word_places: Vec<WordPlace>) -> SolveState {
    if is_solved(&word_places) {
        return SolveState::Solved(word_places);
    }

    let best_candidate = find_best_candidate(&wordlist, &word_places);
    for word in words_that_fit(&wordlist, &best_candidate) {
        let new_word_places = match place_word(&word_places, &best_candidate, &word, &wordlist) {
            PlaceResult::Fits(result) => result,
            PlaceResult::DoesNotFit => continue,
        };
        match solve(wordlist, new_word_places) {
            SolveState::Solved(solution) => return SolveState::Solved(solution),
            SolveState::Unsolved => continue,
        }
    }
    return SolveState::Unsolved;
}

pub(crate) fn is_solved(word_places: &Vec<WordPlace>) -> bool {
    word_places.iter().all(|place| {
        place.chars.iter().all(|c| match c {
            Character::Empty => false,
            Character::Letter(_) => true,
        })
    })
}

pub(crate) fn words_that_fit<'a>(
    wordlist: &'a Vec<impl AsRef<str>>,
    word_place: &WordPlace,
) -> Vec<&'a str> {
    wordlist
        .iter()
        .map(|word| word.as_ref())
        .filter(|word| word.chars().count() == word_place.chars.len())
        .filter(|word| {
            word.chars().enumerate().all(|(i, c)| {
                word_place.chars[i] == Character::Empty
                    || word_place.chars[i] == Character::Letter(c)
            })
        })
        .collect()
}

pub(crate) enum PlaceResult {
    Fits(Vec<WordPlace>),
    DoesNotFit,
}

pub(crate) fn place_word(
    word_places: &Vec<WordPlace>,
    target: &WordPlace,
    word: &str,
    wordlist: &Vec<impl AsRef<str>>,
) -> PlaceResult {
    let mut new_word_places = word_places.clone().to_vec();

    for (i, c) in word.chars().enumerate() {
        let x = target.x + if target.dir == Horizontal { i } else { 0 };
        let y = target.y + if target.dir == Vertical { i } else { 0 };

        for place in new_word_places.iter_mut() {
            for i in 0..place.chars.len() {
                let char_x = place.x + if place.dir == Horizontal { i } else { 0 };
                let char_y = place.y + if place.dir == Vertical { i } else { 0 };

                if char_x == x && char_y == y {
                    if let Character::Letter(l) = place.chars[i] {
                        assert!(l == c, "Overlapping words");
                    } else {
                        place.chars[i] = Character::Letter(c);
                        if words_that_fit(wordlist, place).len() == 0 {
                            return PlaceResult::DoesNotFit;
                        }
                    }
                }
            }
        }
    }
    PlaceResult::Fits(new_word_places)
}

pub(crate) fn find_best_candidate<'a>(
    wordlist: &Vec<impl AsRef<str>>,
    word_places: &'a Vec<WordPlace>,
) -> &'a WordPlace {
    word_places
        .iter()
        .reduce(|a, b| {
            if a.chars.iter().all(|c| c != &Character::Empty) {
                return b;
            }
            if b.chars.iter().all(|c| c != &Character::Empty) {
                return a;
            }
            if a.chars.len() > b.chars.len() {
                return a;
            } else if a.chars.len() < b.chars.len() {
                return b;
            }

            let a_words = words_that_fit(wordlist, a);
            let b_words = words_that_fit(wordlist, b);
            if a_words.len() > b_words.len() {
                a
            } else {
                b
            }
        })
        .expect("Don't call with empty word_places")
}
