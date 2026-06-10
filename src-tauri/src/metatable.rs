// Manage individual setting and meta rows saved in the database.
// For example, timestamp for the latest sync.

use tauri::State;

use crate::{db_schema::with_db, SharedDbState};

pub struct MetaData {}

impl MetaData {
    // Get meta field by key.
    pub fn get(
        id: &str,
        default: &str,
        state: &State<'_, SharedDbState>,
    ) -> Result<String, String> {
        let result = with_db(state, |conn| {
            conn.query_row("SELECT value FROM meta WHERE id = ?1", [&id], |row| {
                row.get::<_, String>(0)
            })
        });

        match result {
            Ok(value) => Ok(value),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(String::from(default)),
            Err(other_error) => Err(other_error.to_string()),
        }
    }

    // Insert or update a meta field.
    pub fn set(id: &str, value: &str, state: &State<'_, SharedDbState>) -> Result<(), String> {
        let result = with_db(state, |conn| {
            conn.execute("INSERT INTO meta (id, value) values(?1, ?2) ON CONFLICT(id) DO UPDATE SET value = excluded.value;", [&id, &value])
        });

        match result {
            Ok(_) => Ok(()),
            Err(other_error) => Err(other_error.to_string()),
        }
    }
}
