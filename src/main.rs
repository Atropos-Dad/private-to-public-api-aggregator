use tide::http::Request;
use tide::{log, prelude::*};
use tide::{Response, StatusCode};
use std::collections::VecDeque;
use std::sync::Mutex;
use std::sync::LazyLock;
use dotenv::dotenv;

// Fixed-size queue of 5 most recently read URLs
static LAST_READ_URLS: LazyLock<Mutex<VecDeque<String>>> = LazyLock::new(|| {
    Mutex::new(VecDeque::with_capacity(5))
});

static API_KEY: LazyLock<String> = LazyLock::new(|| {
    std::env::var("API_KEY").expect("API_KEY must be set.")
});

fn validate_api_key(req: &tide::Request<()>) -> bool {
    let auth_header = req.header("Authorization");
    auth_header.is_some() && auth_header.unwrap().as_str().eq(&format!("Bearer {}", *API_KEY))
}

async fn log_url(mut req: tide::Request<()>) -> tide::Result<Response> {
    // Check for API key in the request headers
    if !validate_api_key(&req) {
        return Ok(Response::new(StatusCode::Unauthorized));
    }
    
    // Read the request body as a string
    let body_str = req.body_string().await?;
    
    // Log the body to the console
    println!("Received webhook: {}", body_str);

    // Add the new URL to the queue, removing oldest if needed
    let mut urls = LAST_READ_URLS.lock().unwrap();
    if urls.len() >= 5 {
        urls.pop_front(); // Remove oldest URL if we have 5 already
    }
    urls.push_back(body_str); // Add the new URL
    
    // Return a response
    let res = Response::new(StatusCode::Ok);
    Ok(res)
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv().ok();
    tide::log::start();
    let mut app = tide::new();
    
    app.at("/").get(|_| async { Ok("API Endpoint Aggregator") });
    app.at("/url-webhook").post(log_url);
    
    println!("Server running on http://localhost:4653");
    app.listen("127.0.0.1:4653").await?;
    Ok(())
}