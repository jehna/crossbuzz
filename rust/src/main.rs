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
        xxxxxxxxx.xxxxx
        ...xxx.xxxx.IPO
        REAKTOR.xxxx...
        xxxxxxxx.JAAHAS
        xxxx.xxxxxxxxxx
        xxxx.xxxxx.xxxx
        xxxx.xxxxx.xxxx
    ";

    let input_places = input_places_from_visual(trim_indent_and_whitespace(input));
    let result = pretty_print(solver::solve(wordlist, input_places));
    println!("{}", result);
}

fn read_wordlist<'a>() -> Vec<String> {
    let filename = "words.txt";
    let contents = std::fs::read_to_string(filename).expect("Could not read file");
    contents.lines().map(|s| s.to_uppercase()).collect()
}

mod direction;
mod word_place;

mod solver;

#[cfg(test)]
mod tests {
    use crate::*;

    fn expect_solved<'a>(input: &str, expected: &str, wordlist: impl AsRef<[&'a str]>) {
        let input_places = input_places_from_visual(trim_indent_and_whitespace(input));
        let result = pretty_print(solver::solve(
            wordlist
                .as_ref()
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
            input_places,
        ));
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
