use super::board::Board;
use super::game::Game;
use super::snake::Snake;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct State {
    pub game: Game,
    pub turn: u16,
    pub board: Board,
    pub you: Snake,
}
