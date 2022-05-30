use actix_web::{get, middleware, post, web, App, HttpServer};
use clap::Parser;
use serde_json::{json, Value};
mod game;
mod heuristic;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser, Debug, Clone)]
#[clap(version)]
struct Config {
    #[clap(short, long, default_value_t = 8080)]
    port: u16,

    #[clap(long, default_value_t = 1.5)]
    hunger_coefficient: f32,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = Config::parse();
    let port = config.port;

    println!("{:?}", config);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .wrap(middleware::Logger::default())
            .service(index)
            .service(start)
            .service(mv)
            .service(end)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

#[get("/")]
async fn index() -> web::Json<Value> {
    web::Json(json!({
        "apiversion": "1",
        "author": "broothie",
        "color": "#DB5527",
        "head": "tongue",
        "tail": "block-bum",
        "version": VERSION,
    }))
}

#[post("/start")]
async fn start() -> String {
    "start".to_string()
}

#[post("/move")]
async fn mv(data: web::Data<Config>, state: web::Json<game::State>) -> web::Json<game::Command> {
    println!(
        "game {}, turn {}: {:?}",
        state.game.id, state.turn, state.game
    );

    let state = state.into_inner();
    let heuristic = heuristic::Heuristic { state: state.clone() };

    let command = heuristic
        .decide(data.hunger_coefficient)
        .unwrap_or_else(|_| game::Command {
            mv: game::Move::Up,
            shout: "whoops!".to_string(),
        });

    println!(
        "game {}, turn {}: {:?} '{}'",
        state.game.id, state.turn, command.mv, command.shout
    );

    web::Json(command)
}

#[post("/end")]
async fn end() -> String {
    "end".to_string()
}
