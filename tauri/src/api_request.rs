pub struct RequestEndpoint {}

impl RequestEndpoint {
    pub fn new(path: &str) -> String {
        let base_url = "http://10.0.2.2:4576";
        format!("{base_url}/v1/{path}")
    }
}

pub mod endpoints {

    pub static SIGN_UP_END_POINT: &str = "auth/register";
    pub static LOG_IN_END_POINT: &str = "auth/sign-in";
}
