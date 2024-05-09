fn main() {}

mod direction {
    #[derive(PartialEq, Clone)]
    pub(crate) enum Direction {
        Horizontal,
        Vertical,
    }
}
use direction::Direction::{self, *};

#[derive(Clone, Copy, PartialEq)]
enum Character {
    Empty,
    Letter(char),
}

impl Default for Character {
    fn default() -> Self {
        Character::Empty
    }
}

#[derive(Clone)]
struct WordPlace {
    chars: Vec<Character>,
    x: usize,
    y: usize,
    dir: Direction,
}

impl WordPlace {
    fn to_string(&self) -> String {
        self.chars
            .iter()
            .map(|c| match c {
                Character::Empty => ' ',
                Character::Letter(l) => *l,
            })
            .collect()
    }
}

enum SolveState {
    Solved(Vec<WordPlace>),
    Unsolved,
}

fn solve(wordlist: &Vec<impl AsRef<str>>, word_places: Vec<WordPlace>) -> SolveState {
    if is_solved(&word_places) {
        return SolveState::Solved(word_places);
    }

    let best_candidate = find_best_candidate(&wordlist, &word_places);
    for word in words_that_fit(&wordlist, &best_candidate) {
        let new_word_places = place_word(&word_places, &best_candidate, &word);
        match solve(wordlist, new_word_places) {
            SolveState::Solved(solution) => return SolveState::Solved(solution),
            SolveState::Unsolved => continue,
        }
    }
    return SolveState::Unsolved;
}

fn is_solved(word_places: &Vec<WordPlace>) -> bool {
    word_places.iter().all(|place| {
        place.chars.iter().all(|c| match c {
            Character::Empty => false,
            Character::Letter(_) => true,
        })
    })
}

fn words_that_fit<'a>(wordlist: &'a Vec<impl AsRef<str>>, word_place: &WordPlace) -> Vec<&'a str> {
    wordlist
        .iter()
        .map(|word| word.as_ref())
        .filter(|word| word.len() == word_place.chars.len())
        .collect()
}

fn place_word(word_places: &Vec<WordPlace>, target: &WordPlace, word: &str) -> Vec<WordPlace> {
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
                    }
                }
            }
        }
    }
    new_word_places
}

fn find_best_candidate<'a>(
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

#[cfg(test)]
mod tests {
    use crate::{solve, Character, SolveState::*, WordPlace, *};

    use self::direction::Direction;

    fn empty_place(direction: Direction) -> WordPlace {
        WordPlace {
            chars: vec![Character::Empty; 5],
            x: 0,
            y: 0,
            dir: direction,
        }
    }

    #[test]
    fn test_places_simple_words() {
        let words = vec!["hello"];
        let places = vec![empty_place(Horizontal), empty_place(Vertical)];

        match solve(&words, places) {
            Solved(result) => {
                assert_eq!(result[0].to_string(), "hello");
                assert_eq!(result[1].to_string(), "hello");
            }
            Unsolved => panic!("Expected Solved, got Unsolved"),
        }
    }
}
