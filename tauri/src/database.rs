use crate::models::DATABASE_MODELS;
use native_db::{db_type, Database};

pub struct BookmarksDatabaseWasm {}

impl BookmarksDatabaseWasm {
    pub fn init() -> Result<Database<'static>, db_type::Error> {
        let db = native_db::Builder::new()
            .create_in_memory(&DATABASE_MODELS)
            .unwrap();
        Ok(db)
    }
}
