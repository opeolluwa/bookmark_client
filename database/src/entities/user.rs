use std::str::FromStr;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// the user information backed up by the embedded database
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: Uuid,
    first_name: String,
    last_name: String,
    access_token: String,
    email: String,
}

impl User {
    pub fn new(first_name: &str, last_name: &str, access_token: &str, email: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            access_token: access_token.to_string(),
            email: email.to_string(),
        }
    }

    pub fn save(&self, conn: &rusqlite::Connection) {
        conn.execute(
            r#"
            INSERT INTO users (id, first_name, last_name, access_token, email) VALUES (?1, ?2, ?3, ?4, ?5)
            "#,
            &[
                &self.id.to_string(),
                &self.first_name,
                &self.last_name,
                &self.access_token,
                &self.email,
            ],
        )
        .unwrap();
    }

    pub fn fetch(conn: &rusqlite::Connection) -> rusqlite::Result<Self> {
        let mut statement = conn.prepare(
            r#"
            SELECT id, first_name, last_name, access_token, email FROM users WHERE id = 1
            "#,
        )?;

        let user = statement.query_row([], |row| {
            let id: String = row.get(0)?;
            Ok(User {
                id: Uuid::from_str(&id).unwrap(),
                first_name: row.get(1)?,
                last_name: row.get(2)?,
                access_token: row.get(3)?,
                email: row.get(4)?,
            })
        })?;

        Ok(user)
    }
}
