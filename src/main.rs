#[macro_use]
extern crate rocket;

use std::collections::HashMap;

use chrono::prelude::*;
use rocket::{fairing::AdHoc, serde::json::Json};
use serde::Serialize;

mod game;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[launch]
fn rocket() -> _ {
	let port = option_env!("PORT")
		.unwrap_or("8000")
		.parse::<u16>()
		.unwrap();

	let cfg = rocket::config::Config::figment()
		.merge(("port", port))
		.merge(("address", "0.0.0.0"));

	rocket::custom(cfg)
		.mount("/", routes![index, start, mv, end])
		.attach(AdHoc::on_response("logger", |req, res| Box::pin(async move {
			println!("{} | {} | {} {}", Utc::now(), res.status(), req.method(), req.uri())
		})))
}

#[get("/")]
fn index() -> Json<HashMap<&'static str, &'static str>> {
	Json(HashMap::from([
		("apiversion", "1"),
		("author", "broothie"),
		("color", "#888888"),
		("head", "tongue"),
		("tail", "block-bum"),
		("version", VERSION),
	]))
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
