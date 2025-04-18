use tide::{log, prelude::*};
use dotenv::dotenv;
use femme::LevelFilter;
use std::env;

mod url_handlers;
mod auth;
mod letterboxd;
mod spotify;
mod cache;

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv().ok();
    
    // Set log level based on environment (default to Info for production)
    let log_level = match env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()).as_str() {
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    };
    tide::log::with_level(log_level);
    
    let mut app = tide::new();
    
    // Get host and port from environment variables or use defaults
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "4653".to_string());
    
    app.at("/").get(|_| async { Ok("API Endpoint Aggregator") });
    app.at("/url-webhook").post(url_handlers::log_url);
    app.at("/url-webhook").get(url_handlers::get_urls);
    app.at("/letterboxd").get(letterboxd::get_letterboxd_movies);
    app.at("/spotify").get(spotify::get_spotify_tracks);
    
    log::info!("Server running on http://{}:{}", host, port);
    app.listen(format!("{}:{}", host, port)).await?;
    Ok(())
}