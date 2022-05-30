use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Game {
    pub id: String,
    pub ruleset: Ruleset,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Ruleset {
    pub name: String,
}
