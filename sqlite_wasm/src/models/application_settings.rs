use crate::database::CreateTable;

#[derive(Debug)]
pub struct ApplicationSettings {
    pub theme: String,
    pub language: String,
}

impl CreateTable for ApplicationSettings {
    fn create_table(connection: rusqlite::Connection) -> rusqlite::Result<()> {
        let query = r#"
CREATE TABLE "application_settings" (
    id INT PRIMARY KEY,
    theme VARCHAR,
    language VARCHAR
)
"#;
        connection.execute(&query, ())?;
        Ok(())
    }
}
