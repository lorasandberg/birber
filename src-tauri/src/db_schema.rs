use axum::Extension;
use axum::Router;
use rusqlite::Connection;
use std::default;
use std::env;
use std::future::Future;
use std::sync::Arc;
use std::sync::Mutex;
use tauri::AppHandle;
use tauri::State;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;

use crate::SharedDbState;

pub type DbPool = Arc<Mutex<Connection>>;

// Setup database file and structure.
pub fn init() -> DbPool {
    let db_path = get_db_path().unwrap();

    let conn = Connection::open(&db_path).expect("Failed to mount database file.");

    conn.execute_batch(include_str!("../src/db/migrations/001.sql"))
        .expect("Failed to run database migrations.");
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

// Returns path to the db file.
pub fn get_db_path() -> Result<String, String> {
    let mut exe_path = env::current_exe().expect("Failed to find current exe path");
    exe_path.pop();
    let db_path = exe_path.join("birder.db");

    Ok(format!("{}", db_path.to_string_lossy()))
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
