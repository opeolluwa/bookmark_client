use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BookmarkCollection {
    pub id: String,
    pub name: String,
    pub description: String,
    pub date_added: String,
    pub last_modified: String,
}
