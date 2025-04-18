use tide::{log, prelude::*};
use tide::{Response, StatusCode};
use std::collections::VecDeque;
use std::sync::Mutex;
use std::sync::LazyLock;
use crate::auth;

static QUEUE_SIZE: usize = 5;

// Fixed-size queue of 5 most recently read URLs
pub static LAST_READ_URLS: LazyLock<Mutex<VecDeque<String>>> = LazyLock::new(|| {
    Mutex::new(VecDeque::with_capacity(QUEUE_SIZE))
});

pub async fn log_url(mut req: tide::Request<()>) -> tide::Result<Response> {
    // Check for API key in the request headers
    if !auth::validate_api_key(&req) {
        return Ok(Response::new(StatusCode::Unauthorized));
    }
    
    // Determine if the request is JSON or raw based on Content-Type header
    let url = if let Some(content_type) = req.header("Content-Type") {
        if content_type.as_str().contains("application/json") {
            // Handle JSON format
            let body: serde_json::Value = req.body_json().await?;
            match body.get("url") {
                Some(url_value) => {
                    if let Some(url_str) = url_value.as_str() {
                        url_str.to_string()
                    } else {
                        return Ok(Response::builder(StatusCode::BadRequest)
                            .body(json!({"error": "Invalid URL format in JSON"}))
                            .build());
                    }
                },
                None => {
                    return Ok(Response::builder(StatusCode::BadRequest)
                        .body(json!({"error": "Missing 'url' field in JSON"}))
                        .build());
                }
            }
        } else {
            // Handle raw format
            req.body_string().await?
        }
    } else {
        // Default to raw format if no Content-Type header
        req.body_string().await?
    };
    
    // Add the new URL to the queue, removing oldest if needed
    let mut urls = LAST_READ_URLS.lock().unwrap();
    // Log the body and current URLs
    log::info!("Received webhook: {}", url);
    
    // If at capacity, remove oldest before adding new one
    log::debug!("Current queue length: {}", urls.len());
    if urls.len() >= QUEUE_SIZE {
        log::debug!("Removing oldest URL: {:?}", urls.front());
        urls.pop_front();
    }
    urls.push_back(url); // Add the new URL

    log::debug!("The list of updated webhooks: {:#?}", urls);

    // Return a response
    let res = Response::new(StatusCode::Ok);
    Ok(res)
}

pub async fn get_urls(req: tide::Request<()>) -> tide::Result<Response> {
    // Check for API key in the request headers
    if !auth::validate_api_key(&req) {
        return Ok(Response::new(StatusCode::Unauthorized));
    }

    // Get the URLs from the queue
    let urls = LAST_READ_URLS.lock().unwrap();
    let urls_vec: Vec<String> = urls.iter().cloned().collect();
    let json = json!({ "urls": urls_vec });
    let mut res = Response::new(StatusCode::Ok);
    res.set_content_type("application/json");
    res.set_body(json);
    Ok(res)
} 