use crate::direction::Direction::{Horizontal, Vertical};
use crate::word_place::Character;
use crate::word_place::WordPlace;

#[derive(PartialEq, Debug)]
pub(crate) enum SolveState {
    Solved(Vec<WordPlace>),
    Unsolved,
}

#[derive(Debug, Clone)]
pub(crate) struct WordPlaceWithPossibleWords<'wl> {
    place: WordPlace,
    possible_words: Vec<&'wl String>,
}

pub(crate) fn solve<'wl>(wordlist: Vec<String>, word_places: Vec<WordPlace>) -> SolveState {
    let completed_words_from_places = to_completed_words_from_places(&word_places);
    let full_wordlist = completed_words_from_places
        .iter()
        .chain(wordlist.iter())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    let word_places_with_possible_words = word_places_to_possible_words(full_wordlist, word_places);

    solve_cached(word_places_with_possible_words)
}

fn to_completed_words_from_places(word_places: &Vec<WordPlace>) -> Vec<String> {
    word_places
        .iter()
        .filter(|place| place.chars.iter().all(|c| c != &Character::Empty))
        .map(|place| {
            place
                .chars
                .iter()
                .map(|c| match c {
                    Character::Empty => panic!("Empty character"),
                    Character::Letter(l) => *l,
                })
                .collect()
        })
        .collect()
}

fn word_places_to_possible_words<'wl>(
    wordlist: Vec<&'wl String>,
    word_places: Vec<WordPlace>,
) -> Vec<WordPlaceWithPossibleWords<'wl>> {
    word_places
        .into_iter()
        .map(|place| {
            let possible_words = words_that_fit(&wordlist, &place);
            WordPlaceWithPossibleWords {
                place,
                possible_words,
            }
        })
        .collect()
}

fn possible_places_to_word_places(
    word_places_with_possible_words: Vec<WordPlaceWithPossibleWords>,
) -> Vec<WordPlace> {
    word_places_with_possible_words
        .into_iter()
        .map(|place| WordPlace {
            chars: place
                .possible_words
                .get(0)
                .expect("One word")
                .chars()
                .map(Character::Letter)
                .collect(),
            x: place.place.x,
            y: place.place.y,
            dir: place.place.dir,
        })
        .collect()
}

fn solve_cached(word_places_with_possible_words: Vec<WordPlaceWithPossibleWords>) -> SolveState {
    if is_solved(&word_places_with_possible_words) {
        return SolveState::Solved(possible_places_to_word_places(
            word_places_with_possible_words,
        ));
    }

    let best_candidate = find_best_candidate(&word_places_with_possible_words);
    for word in best_candidate.possible_words.iter() {
        let new_word_places = match place_word(
            &word_places_with_possible_words,
            &best_candidate.place,
            &word,
        ) {
            PlaceResult::Fits(result) => result,
            PlaceResult::DoesNotFit => continue,
        };
        match solve_cached(new_word_places) {
            SolveState::Solved(solution) => return SolveState::Solved(solution),
            SolveState::Unsolved => continue,
        }
    }
    return SolveState::Unsolved;
}

pub(crate) fn is_solved(word_places: &Vec<WordPlaceWithPossibleWords>) -> bool {
    word_places
        .iter()
        .all(|place| place.place.chars.iter().all(|c| c != &Character::Empty))
}

pub(crate) fn words_that_fit<'wl>(
    wordlist: &Vec<&'wl String>,
    word_place: &WordPlace,
) -> Vec<&'wl String> {
    wordlist
        .iter()
        .filter(|word| word.chars().count() == word_place.chars.len())
        .filter(|word| {
            word.chars().enumerate().all(|(i, c)| {
                word_place.chars[i] == Character::Empty
                    || word_place.chars[i] == Character::Letter(c)
            })
        })
        .map(|word| *word)
        .collect()
}

pub(crate) enum PlaceResult<'wl> {
    Fits(Vec<WordPlaceWithPossibleWords<'wl>>),
    DoesNotFit,
}

pub(crate) fn place_word<'wl>(
    word_places_with_possible_words: &Vec<WordPlaceWithPossibleWords<'wl>>,
    target: &WordPlace,
    word: &str,
) -> PlaceResult<'wl> {
    let mut new_word_places = word_places_with_possible_words.clone().to_vec();

    for (i, c) in word.chars().enumerate() {
        let x = target.x + if target.dir == Horizontal { i } else { 0 };
        let y = target.y + if target.dir == Vertical { i } else { 0 };

        for place in new_word_places.iter_mut() {
            for i in 0..place.place.chars.len() {
                let char_x = place.place.x + if place.place.dir == Horizontal { i } else { 0 };
                let char_y = place.place.y + if place.place.dir == Vertical { i } else { 0 };

                if char_x == x && char_y == y {
                    if let Character::Letter(l) = place.place.chars[i] {
                        assert!(l == c, "Overlapping words");
                    } else {
                        place.place.chars[i] = Character::Letter(c);
                        place.possible_words.retain(|word| {
                            word.chars()
                                .enumerate()
                                .all(|(i, c)| match place.place.chars[i] {
                                    Character::Empty => true,
                                    Character::Letter(l) => l == c,
                                })
                        });
                        if place.possible_words.len() == 0 {
                            return PlaceResult::DoesNotFit;
                        }
                    }
                }
            }
        }
    }
    PlaceResult::Fits(new_word_places)
}

pub(crate) fn find_best_candidate<'vec, 'wl>(
    word_places_with_possible_words: &'vec Vec<WordPlaceWithPossibleWords<'wl>>,
) -> &'vec WordPlaceWithPossibleWords<'wl> {
    word_places_with_possible_words
        .iter()
        .reduce(|a, b| {
            if a.place.chars.iter().all(|c| c != &Character::Empty) {
                return b;
            }
            if b.place.chars.iter().all(|c| c != &Character::Empty) {
                return a;
            }

            if a.possible_words.len() < b.possible_words.len() {
                a
            } else {
                b
            }
        })
        .expect("Don't call with empty word_places")
}
