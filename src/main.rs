#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use serde::Serialize;

mod game;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize)]
struct Config {
	apiversion: String,
	author: String,
	color: String,
	head: String,
	tail: String,
	version: String,
}

#[launch]
fn rocket() -> _ {
	rocket::build().mount("/", routes![config, start, mv, end])
}

#[get("/")]
fn config() -> Json<Config> {
	Json(Config {
		apiversion: "1".to_string(),
		author: "broothie".to_string(),
		color: "#888888".to_string(),
		head: "tongue".to_string(),
		tail: "block-bum".to_string(),
		version: VERSION.to_string(),
	})
}

#[post("/start")]
fn start() {}

#[post("/move", data = "<state>")]
fn mv(state: Json<game::State>) -> Json<game::Command> {
	let mv = state.decide().unwrap_or(game::Move::Up);
	let command = game::Command { mv };
	println!("turn {} {:?}", state.turn, command);

	Json(command)
}

#[post("/end")]
fn end() {}
