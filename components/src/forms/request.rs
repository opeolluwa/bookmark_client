pub struct RequestEndpoint {}

pub static EMULATOR_BASE_DIR: &'static str = "http://10.0.2.2:4576";
pub static STAGING_BASE_DIR: &'static str = "https://vulkan-8vfs.shuttle.app";
pub static DEVELOPMENT_BASE_DIR: &'static str = "http://localhost:4576";

impl RequestEndpoint {
    pub fn new(path: &str) -> String {
        let base_url = EMULATOR_BASE_DIR;
        format!("{base_url}/v1/{path}")
    }
}
