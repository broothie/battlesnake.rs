use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Game {
    pub id: String,
    pub ruleset: Ruleset,
}

#[derive(Deserialize, Debug)]
pub struct Ruleset {
    pub name: String,
}
