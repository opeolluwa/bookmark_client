pub struct RequestEndpoint {}

impl RequestEndpoint {
    pub fn new(path: &str) -> String {
        let base_url = "https://bookmark-server-rm6f.onrender.com";
        format!("{base_url}/v1/{path}")
    }
}
