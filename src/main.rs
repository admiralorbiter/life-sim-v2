mod models;
mod engine;
mod data_loader;
mod api;

use actix_web::{App, HttpServer, web};
use actix_files as fs;
use std::path::PathBuf;
use std::sync::Mutex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load game data from JSON files
    let data_dir = PathBuf::from("data");
    let game_data = data_loader::GameData::load_from_dir(&data_dir)
        .expect("Failed to load game data from data/ directory");

    let game_data = web::Data::new(game_data);

    // Shared mutable game state (one game per process for MVP)
    let app_state = web::Data::new(api::routes::AppState {
        game: Mutex::new(None),
        rng: Mutex::new(None),
    });

    println!("\n🎮 Life Roguelite server starting...");
    println!("   Open http://localhost:8080 in your browser\n");

    HttpServer::new(move || {
        App::new()
            .app_data(game_data.clone())
            .app_data(app_state.clone())
            // API routes
            .configure(api::routes::configure)
            // Static files (index.html, css, js)
            .service(fs::Files::new("/", "static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
