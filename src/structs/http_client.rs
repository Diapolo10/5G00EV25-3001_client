/// Creates an instance of reqwest client and stores base url
pub struct HttpClient {
    pub client: reqwest::blocking::Client,
    pub base_url: String,
}

impl Default for HttpClient {
    fn default() -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            base_url: "http://127.0.0.1:8080/".to_owned(),
        }
    }
}
