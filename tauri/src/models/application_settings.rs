use native_db::{native_db, ToKey};
use native_model::{native_model, Model};
use serde::{Deserialize, Serialize};

pub type AppSettings = v1::ApplicationSettings;

pub mod v1 {
    use super::*;
    #[derive(Debug, Serialize, Deserialize)]
    #[native_model(id = 1, version = 1)]
    #[native_db]
    pub struct ApplicationSettings {
        #[primary_key]
        pub id: u8,
        pub theme: String,
        pub language: String,
    }
}
