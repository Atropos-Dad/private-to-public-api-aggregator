use std::sync::LazyLock;
use tide::Request;

pub static API_KEY: LazyLock<String> = LazyLock::new(|| {
    std::env::var("API_KEY").expect("API_KEY must be set.")
});

pub fn validate_api_key(req: &Request<()>) -> bool {
    let auth_header = req.header("Authorization");
    auth_header.is_some() && auth_header.unwrap().as_str().eq(&format!("Bearer {}", *API_KEY))
} 