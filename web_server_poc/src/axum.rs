pub mod memory;
use memory::get_process_memory;

use std::fs;
use axum::{ response::Html, routing::get, Router };


async fn index() -> Html<String> {
    let mem_before = get_process_memory();
    let contents = match fs::read_to_string("index.html") {
        Ok(data) => data,
        Err(_) => "Error loading file".to_string(),
    };

    let mem_after = get_process_memory();
    println!("API call - Before : {} kB, After : {} kB", mem_before, mem_after);
    Html(contents)
}

#[tokio::main]
async fn main() {
    // Memory before starting web-server
    let mem_before = get_process_memory();
    println!("Memory Before starting server {} kB", mem_before);
    // build our application with a single route
    let app = Router::new().route("/", get(index));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    let mem_after = get_process_memory();
    println!("Memory After starting server {} kB", mem_after);
    axum::serve(listener, app).await.unwrap();
}
