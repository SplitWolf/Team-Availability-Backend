use api::DynAvailStore;
use axum::{
    http::StatusCode, routing::get_service, Router
};
use tower_http::services:: { ServeDir, ServeFile};
use data::PostgresAvailablityStore;
use sqlx::postgres::PgPoolOptions;
mod model;
mod data;
mod api;
mod error;

#[tokio::main]
async fn main() {

    let db_url = std::env::var("DATABASE_URL").expect("Database url not found");
    println!("{}",db_url);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        //TODO: Parse at runtime
        .connect(db_url.as_str())
        .await
        .expect("can connect to db");


    //TODO: Check this type with vid
    let store = std::sync::Arc::new(PostgresAvailablityStore::new(pool)) as DynAvailStore;
    let static_file_serve = get_service(ServeDir::new(env!("STATIC_DIR")).fallback(ServeFile::new(env!("STATIC_FILE")))).handle_error(|_| async move {
        (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
    });

    let app = Router::new()
        .nest("/api", api::api_routes(store))
        .fallback(static_file_serve);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
