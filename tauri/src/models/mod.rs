pub mod application_settings;
pub mod bookmark_collection;
pub mod bookmark_entries;

use native_db::Models;
use once_cell::sync::Lazy;

pub static DATABASE_MODELS: Lazy<Models> = Lazy::new(|| {
    let mut models = Models::new();
    models
        .define::<application_settings::v1::ApplicationSettings>()
        .unwrap();
    models
});
