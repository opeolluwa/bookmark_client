use crate::models::application_settings::AppSettings;
use native_db::*;

pub struct BookmarksDatabaseWasm {}

impl BookmarksDatabaseWasm {
    pub fn init() -> Result<(), db_type::Error> {
        let models = Self::models();
        let db_connections = Self::db_connection()?;

        let db = Builder::new().create_in_memory(&models)?;
        Ok(())
    }

    pub fn db_connection() -> Result<Database<'static>, db_type::Error> {
        
    }

    pub fn models() -> Models {
        let mut models = Models::new();
        let models = models.define::<AppSettings>().unwrap();
        models
    }
}
