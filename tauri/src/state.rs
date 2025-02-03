use std::sync::Mutex;

use bookmark_local_database::rusqlite::Connection;

#[derive(Debug)]
pub struct AppState {
    pub conn: Mutex<Connection>,
}
