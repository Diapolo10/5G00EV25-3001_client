/// Creates an instance of reqwest client and stores base url
// This could be made to a singleton but multiple instances might be needed for testing
// Also using the client as singleton seemed like a lot of work
pub struct HttpClient {
    pub client: reqwest::blocking::Client,
    pub base_url: String,
}

impl Default for HttpClient {
    fn default() -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            base_url: "http://127.0.0.1:11037/".to_owned(),
        }
    }
}
