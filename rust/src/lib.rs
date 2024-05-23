use std::thread;

use wasm_bindgen::prelude::*;
mod string_to_places;
use string_to_places::*;
mod direction;
mod solver;
mod word_place;

#[wasm_bindgen]
pub fn solve(input: String, wordlist: Vec<String>) -> String {
    let builder = thread::Builder::new()
        .name("reductor".into())
        .stack_size(1024 * 1024 * 1024);

    let handler = builder.spawn(|| do_solve(input, wordlist)).unwrap();

    let result = handler.join().unwrap();
    result
}

pub fn do_solve(input: String, wordlist: Vec<String>) -> String {
    let input_places = input_places_from_visual(trim_indent_and_whitespace(input.as_str()));
    pretty_print(solver::solve(wordlist, input_places))
}
