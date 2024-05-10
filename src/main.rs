mod string_to_places;
use std::thread;

use string_to_places::*;

fn main() {
    let builder = thread::Builder::new()
        .name("reductor".into())
        .stack_size(1024 * 1024 * 1024);

    let handler = builder
        .spawn(|| {
            do_solve();
        })
        .unwrap();

    handler.join().unwrap();
}

fn do_solve() {
    let wordlist = read_wordlist();
    let input = "
        xxxx
        xxxx
        xxxx
        xxxx
    ";

    let input_places = input_places_from_visual(trim_indent_and_whitespace(input));
    let result = pretty_print(solver::solve(&wordlist, input_places));
    println!("{}", result);
}

fn read_wordlist<'a>() -> Vec<String> {
    let filename = "words.txt";
    let contents = std::fs::read_to_string(filename).expect("Could not read file");
    contents.lines().map(|s| s.to_uppercase()).collect()
}

mod direction;
use direction::Direction::{self};
use word_place::{Character, WordPlace};
mod word_place;

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

mod solver;

#[cfg(test)]
mod tests {
    use self::direction::Direction;
    use crate::{
        solver::solve, solver::SolveState::*, word_place::Character, word_place::WordPlace, *,
    };

    fn empty_place(direction: Direction) -> word_place::WordPlace {
        word_place::WordPlace {
            chars: vec![Character::Empty; 5],
            x: 0,
            y: 0,
            dir: direction,
        }
    }

    fn empty_place_pos(x: usize, y: usize, direction: Direction) -> word_place::WordPlace {
        word_place::WordPlace {
            chars: vec![Character::Empty; 5],
            x,
            y,
            dir: direction,
        }
    }

    fn assert_matches(result: solver::SolveState, expected: Vec<&str>) {
        match result {
            Solved(solution) => {
                for (i, word) in expected.iter().enumerate() {
                    assert_eq!(solution[i].to_string(), *word);
                }
            }
            Unsolved => panic!("Expected Solved, got Unsolved"),
        }
    }

    fn expect_solved<'a>(input: &str, expected: &str, wordlist: impl AsRef<[&'a str]>) {
        let input_places = input_places_from_visual(trim_indent_and_whitespace(input));
        let result = pretty_print(solver::solve(&wordlist.as_ref().to_vec(), input_places));
        let expected_trimmed = trim_indent_and_whitespace(expected);
        assert_eq!(result, expected_trimmed);
    }

    #[test]
    fn places_simple_words() {
        expect_solved(
            "
                xxxxx
                x....
                x....
                x....
                x....
            ",
            "
                HELLO
                E....
                L....
                L....
                O....
            ",
            ["HELLO"],
        );
    }

    #[test]
    fn discards_words_that_dont_fit() {
        expect_solved(
            "
                xxxxx
                x....
                x....
                x....
                x....
            ",
            "
                WORLD
                O....
                R....
                L....
                D....
            ",
            ["NOPE", "WORLD"],
        );
    }

    #[test]
    fn finds_different_word_to_match_place() {
        expect_solved(
            "
                xxxxx
                .x...
                .x...
                .x...
                .x...
            ",
            "
                HELLO
                .M...
                .P...
                .T...
                .Y...
            ",
            ["HELLO", "EMPTY"],
        );
    }

    #[test]
    fn does_backtrace() {
        expect_solved(
            "
                xxxxx
                ....x
                ....x
                xxxxx
                ....x
            ",
            "
                HELLO
                ....T
                ....H
                THERE
                ....R
            ",
            ["HELLO", "ODDLY", "OTHER", "THERE"],
        );
    }

    #[test]
    fn returns_unsolved_if_cannot_be_solved() {
        expect_solved(
            "
                xxxxx
                ....x
                ....x
                xxxxx
                ....x
            ",
            "Unsolved",
            ["HELLO", "ODDLY", "THERE"],
        );
    }

    #[test]
    fn needs_to_result_in_correct_words_in_all_directions() {
        expect_solved(
            "
                xxx
                xxx
                xxx
            ",
            "
                ODD
                DIY
                DYE",
            ["ODD", "DIY", "DYE"],
        );
    }
}
