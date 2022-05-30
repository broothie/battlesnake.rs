use super::mv::Move;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Command {
    pub mv: Move,
    pub shout: String,
}
