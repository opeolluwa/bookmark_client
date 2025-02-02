pub struct RequestEndpoint {}

pub static EMULATOR_BASE_DIR: &'static str = "http://10.0.2.2:4576";
pub static STAGING_BASE_DIR: &'static str = "https://vulkan-8vfs.shuttle.app";
pub static DEVELOPMENT_BASE_DIR: &'static str = "http://localhost:4576";

impl RequestEndpoint {
    pub fn new(path: &str) -> String {
        // let base_url = match std::env::var("BASE_URL") {
        //     Ok(base_url) => base_url,
        //     Err(_) => {
        //         if cfg!(target_os = "android") {
        //             EMULATOR_BASE_DIR.to_string()
        //         } else {
        //             DEVELOPMENT_BASE_DIR.to_string()
        //         }
        //     }
        // };
        format!(
            "{base_url}/v1/{path}",
            base_url = EMULATOR_BASE_DIR,
            path = path
        )
    }
}
