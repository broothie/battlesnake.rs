use serde::Serialize;

#[derive(Serialize, PartialEq, Eq, Copy, Clone, Debug, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    pub fn all() -> Vec<Self> {
        vec![Move::Up, Move::Down, Move::Left, Move::Right]
    }
}
