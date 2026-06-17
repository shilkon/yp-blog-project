use blog_client::{HttpClient, Transport};

use std::sync::OnceLock;

const API_URL: &str = "http://localhost:8080";

static CLIENT: OnceLock<Transport> = OnceLock::new();

pub fn client() -> &'static Transport {
    CLIENT.get_or_init(|| {
        Transport::Http(
            HttpClient::new(API_URL.to_string()).unwrap()
        )
    })
}