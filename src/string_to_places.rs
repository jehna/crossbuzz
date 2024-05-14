use crate::direction::Direction::{self, *};
use crate::word_place::WordPlace;
use crate::{
    solver::SolveState::{self, *},
    word_place::Character,
};

const MIN_WORD_LENGTH: usize = 3;

pub(crate) fn trim_indent_and_whitespace(input: &str) -> String {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>()
        .join("\n")
}

pub(crate) fn input_places_from_visual(input: String) -> Vec<WordPlace> {
    let rows = to_rows_chars(to_rows(input.clone()));
    let columns = to_columns_chars(to_columns(input));
    // Both rows and columns concat
    rows.into_iter()
        .chain(columns.into_iter())
        .flat_map(to_world_places)
        .filter(|place| place.chars.len() >= MIN_WORD_LENGTH)
        .collect()
}

pub(crate) struct Char {
    pub(crate) x: usize,
    pub(crate) y: usize,
    pub(crate) c: char,
    pub(crate) dir: Direction,
}

pub(crate) fn to_rows(input: String) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

pub(crate) fn to_rows_chars(input: Vec<String>) -> Vec<Vec<Char>> {
    input
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| Char {
                    x,
                    y,
                    c,
                    dir: Horizontal,
                })
                .collect()
        })
        .collect()
}

pub(crate) fn to_columns_chars(columns: Vec<String>) -> Vec<Vec<Char>> {
    columns
        .iter()
        .enumerate()
        .map(|(x, column)| {
            column
                .chars()
                .enumerate()
                .map(|(y, c)| Char {
                    x,
                    y,
                    c,
                    dir: Vertical,
                })
                .collect()
        })
        .collect()
}

pub(crate) fn to_columns(input: String) -> Vec<String> {
    let mut columns = vec!["".to_string(); input.lines().next().unwrap().len()];
    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            columns[i].push(c);
        }
    }
    columns
}

pub(crate) fn to_world_places(chars: Vec<Char>) -> Vec<WordPlace> {
    let mut places = vec![];
    let mut current_place = None;
    for c in chars {
        match c.c {
            '.' => {
                if let Some(place) = current_place {
                    places.push(place);
                }
                current_place = None;
            }
            'x' => {
                if current_place.is_none() {
                    current_place = Some(WordPlace {
                        chars: vec![Character::Empty],
                        x: c.x,
                        y: c.y,
                        dir: c.dir,
                    });
                } else {
                    current_place.as_mut().unwrap().chars.push(Character::Empty);
                }
            }
            letter => {
                if current_place.is_none() {
                    current_place = Some(WordPlace {
                        chars: vec![Character::Letter(letter)],
                        x: c.x,
                        y: c.y,
                        dir: c.dir,
                    });
                } else {
                    current_place
                        .as_mut()
                        .unwrap()
                        .chars
                        .push(Character::Letter(letter));
                }
            }
        }
    }
    if let Some(place) = current_place {
        places.push(place);
    }
    places
}

pub(crate) fn pretty_print(result: SolveState) -> String {
    let mut output = "".to_string();
    match result {
        Solved(solution) => {
            let max_x = solution
                .iter()
                .map(|p| p.x + if p.dir == Vertical { 0 } else { p.chars.len() })
                .max()
                .unwrap();
            let max_y = solution
                .iter()
                .map(|p| p.y + if p.dir == Vertical { p.chars.len() } else { 0 })
                .max()
                .unwrap();

            for y in 0..max_y {
                for x in 0..max_x {
                    let mut found = false;
                    for place in solution.iter() {
                        let chars_x = place.x..=if place.dir == Horizontal {
                            place.x + place.chars.len() - 1
                        } else {
                            place.x
                        };
                        let chars_y = place.y..=if place.dir == Vertical {
                            place.y + place.chars.len() - 1
                        } else {
                            place.y
                        };

                        if chars_x.contains(&x) && chars_y.contains(&y) {
                            let char_index = if place.dir == Horizontal {
                                x - place.x
                            } else {
                                y - place.y
                            };
                            output += match place.chars[char_index] {
                                Character::Empty => '_',
                                Character::Letter(l) => l,
                            }
                            .to_string()
                            .as_str();

                            found = true;
                            break;
                        }
                    }
                    if !found {
                        output += ".";
                    }
                }
                output += "\n";
            }
        }
        Unsolved => output += "Unsolved",
    };
    output.trim().to_string()
}
