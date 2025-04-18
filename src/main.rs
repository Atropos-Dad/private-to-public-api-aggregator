use tide::{log, prelude::*};
use dotenv::dotenv;
use femme::LevelFilter;

mod url_handlers;
mod auth;
mod letterboxd;
mod spotify;

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv().ok();
    tide::log::with_level(LevelFilter::Debug);
    
    let mut app = tide::new();
    
    let host = "127.0.0.1";
    let port = "4653";
    app.at("/").get(|_| async { Ok("API Endpoint Aggregator") });
    app.at("/url-webhook").post(url_handlers::log_url);
    app.at("/url-webhook").get(url_handlers::get_urls);
    app.at("/letterboxd").get(letterboxd::get_letterboxd_movies);
    app.at("/spotify").get(spotify::get_spotify_tracks);
    log::info!("Server running on http://{}:{}", host, port);
    app.listen(format!("{}:{}", host, port)).await?;
    Ok(())
}