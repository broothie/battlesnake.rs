use actix_web::{get, middleware, post, web, App, HttpServer, Responder};
use clap::Parser;
use serde_json::json;

mod game;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[clap(version)]
struct Args {
    #[clap(short, long, default_value_t = 8080)]
    port: u16,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
            .service(start)
            .service(mv)
            .service(end)
    })
    .bind(("0.0.0.0", args.port))?
    .run()
    .await
}

#[get("/")]
async fn index() -> impl Responder {
    web::Json(json!({
        "apiversion": "1",
        "author": "broothie",
        "color": "#888888",
        "head": "tongue",
        "tail": "block-bum",
        "version": VERSION,
    }))
}

#[post("/start")]
async fn start() -> impl Responder {
    "start"
}

#[post("/move")]
async fn mv(state: web::Json<game::State>) -> impl Responder {
    let mv = state.decide().unwrap_or(game::Move::Up);

    web::Json(json!({ "move": mv }))
}

#[post("/end")]
async fn end() -> impl Responder {
    "end"
}
