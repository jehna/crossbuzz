use crate::direction::Direction;

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum Character {
    Empty,
    Letter(char),
}

impl Default for Character {
    fn default() -> Self {
        Character::Empty
    }
}

#[derive(Clone, PartialEq, Debug)]
pub(crate) struct WordPlace {
    pub(crate) chars: Vec<Character>,
    pub(crate) x: usize,
    pub(crate) y: usize,
    pub(crate) dir: Direction,
}
