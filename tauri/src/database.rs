use crate::models::{application_settings::AppSettings, DATABASE_MODELS};
use bookmark_components::settings::Language;
use native_db::{db_type, Database};
pub struct BookmarksDatabaseWasm {}

impl BookmarksDatabaseWasm {
    pub fn init() -> Result<Database<'static>, db_type::Error> {
        let db = native_db::Builder::new()
            .create_in_memory(&DATABASE_MODELS)
            .unwrap();

        let read_write_trnx = db
            .rw_transaction()
            .expect("unable to create the default app settings");

        let _ = read_write_trnx.insert(AppSettings {
            id: 1u8,
            theme: "light".to_string(),
            language: Language::default().to_string(),
        });

        read_write_trnx.commit().expect("failed to commit");
        Ok(db)
    }
}
