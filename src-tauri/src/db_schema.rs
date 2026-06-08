use axum::Extension;
use axum::Router;
use rusqlite::types::Value;
use rusqlite::Connection;
use rusqlite::Row;
use std::collections::HashMap;
use std::default;
use std::env;
use std::future::Future;
use std::sync::Arc;
use std::sync::Mutex;
use tauri::AppHandle;
use tauri::State;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;

use crate::get_db_path;
use crate::SharedDbState;

pub type DbPool = Arc<Mutex<Connection>>;

// Setup database file and structure.
pub fn init() -> DbPool {
    let db_path = get_db_path();

    println!("Path to DB file is: {}", db_path);

    let conn = Connection::open(&db_path).expect("Failed to mount database file.");

    // conn.execute_batch(include_str!("../src/db/migrations/001.sql"))
    //     .expect("Failed to run database migrations.");
    // conn.execute(
    //     "INSERT INTO meta (id, value) VALUES ('init_check', 'verified');",
    //     [],
    // )
    // .expect("Failed to init check meta table.");

    // Setup database microserver
    let shared_db_pool: DbPool = Arc::new(Mutex::new(conn));

    shared_db_pool.clone()
}

// Start a microserver that hosts the local server file.
// (Otherwise Tauri has trouble opening local files on Windows with absolute paths)
pub fn boot_server(shared_db_pool: DbPool) {
    let axum_pool = shared_db_pool.clone();

    tauri::async_runtime::spawn(async move {
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);

        let app = Router::new()
            // .route("/api/meta", axum::routing::get(get_meta_via_http))
            .layer(Extension(axum_pool))
            .layer(cors);

        let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
            .await
            .unwrap();
        println!("Database service up on http://127.0.0.1:8080");
        axum::serve(listener, app).await.unwrap();
    });
}

// Parses DB connection from Tauri state management.
/** Use:
 * with_db(&state, |conn| {
 *  conn.execute(...);
 * })
 * or
 * let result = with_db(&state, |conn| {
 * conn.execute(...)
 * });
 */
pub fn with_db<F, R>(state: &State<'_, SharedDbState>, action: F) -> R
where
    F: FnOnce(&Connection) -> R,
{
    let guard = state.0.lock().expect("Failed to lock database Mutex");
    action(&guard)
}

// A monster function for cleaner DB interface.
// Given an SQL statement and parameters, returns all rows from Database unparsed.
// Opens and closes DB connection.
pub fn db_get_rows(
    sql: &str,
    params: &[(&str, &dyn rusqlite::ToSql)],
    state: &State<'_, SharedDbState>,
) -> Result<Vec<HashMap<String, String>>, String> {
    with_db(state, |conn| {
        let mut query = conn.prepare(sql).map_err(|e| e.to_string())?;

        let columns: Vec<String> = query
            .column_names()
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        let rows_iter = query
            .query_map(params, |row| {
                let mut row_map = HashMap::new();
                for (i, col_name) in columns.iter().enumerate() {
                    let val: String = db_get_row_value_as_string(row, i);
                    row_map.insert(col_name.clone(), val);
                }
                Ok(row_map)
            })
            .map_err(|e| e.to_string())?;

        let mut results = Vec::new();
        for row_res in rows_iter {
            results.push(row_res.map_err(|e| e.to_string())?);
        }

        Ok(results)
    })
}

fn db_get_row_value_as_string(row: &Row, index: usize) -> String {
    match row.get::<_, Value>(index).unwrap() {
        Value::Null => "".to_string(),
        Value::Integer(num) => num.to_string(),
        Value::Real(num) => num.to_string(),
        Value::Text(txt) => txt,
        _ => panic!("Unsupported DB field type!"),
    }
}

/**
 * Get exactly one row from DB.
 * For situations where you know there's one result only, like WHERE id = x
 */
pub fn db_get_row(
    sql: &str,
    params: &[(&str, &dyn rusqlite::ToSql)],
    state: &State<'_, SharedDbState>,
) -> Result<HashMap<String, String>, String> {
    println!("SQL: {}", sql);
    let result = db_get_rows(sql, params, state)?;

    if result.len() != 1 {
        return Err(format!(
            "Used db_get_row but row count is not 1. (Row count is {})",
            result.len()
        ));
    }

    Ok(result[0].clone())
}

pub fn db_insert_row(
    sql: &str,
    params: &[(&str, &dyn rusqlite::ToSql)],
    state: &State<'_, SharedDbState>,
) -> Result<i64, String> {
    let result = with_db(state, |conn| {
        conn.execute(sql, params).map_err(|e| e.to_string())?;
        Ok::<i64, String>(conn.last_insert_rowid())
    })?;
    Ok(result)
}

pub fn db_execute(
    sql: &str,
    params: &[(&str, &dyn rusqlite::ToSql)],
    state: &State<'_, SharedDbState>,
) -> Result<(), String> {
    with_db(state, |conn| {
        conn.execute(sql, params).map_err(|e| e.to_string())?;
        Ok::<(), String>(())
    })?;
    Ok(())
}

/**
 * Helper function to get values from a query which returns only a single column.
 */
pub fn db_get_values(
    sql: &str,
    params: &[(&str, &dyn rusqlite::ToSql)],
    state: &State<'_, SharedDbState>,
) -> Result<Vec<String>, String> {
    Ok(db_get_rows(sql, params, &state)?
        .iter()
        .map(|row| row.values().next().unwrap().clone())
        .collect())
}

/**
 * Fetches a single value from a query.
 * Panics if the query returns multiple rows, zero rows, multiple columns, or zero columns.
 */
pub fn db_get_value(
    sql: &str,
    params: &[(&str, &dyn rusqlite::ToSql)],
    state: &State<'_, SharedDbState>,
) -> Result<String, String> {
    let result = db_get_values(sql, params, state)?;

    if result.len() != 1 {
        panic!(
            "Fetching a single value but received {} values.",
            result.len()
        );
    }

    Ok(result.get(0).unwrap().clone())
}
