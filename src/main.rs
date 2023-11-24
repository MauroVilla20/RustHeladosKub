use actix_web::{get, web, App, HttpServer};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;


mod todolist;
use todolist::services;
struct AppState {
    todolist_entries: Mutex<Vec<TodolistEntry>>
}

#[derive(Serialize, Deserialize, Clone)]
struct TodolistEntry {
    id: i32,
    date: i64,
    title: String,
}
#[get("/")]
async fn index() -> String {
    "Rust API Helados [VILLA MARTINEZ MAURO MARIN] ".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState {
        todolist_entries: Mutex::new(vec![])
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(index)
            .configure(services::config)
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}