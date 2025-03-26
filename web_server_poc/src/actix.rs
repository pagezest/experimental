pub mod memory;
use memory::get_process_memory;

use std::{ fs, io };
use actix_web::{ get, App, HttpResponse, HttpServer, Responder };

#[get("/")]
async fn index() -> impl Responder {
    let mem_before = get_process_memory();
    let contents = match fs::read_to_string("index.html") {
        Ok(data) => data,
        Err(_) => "Error loading file".to_string(),
    };
    let mem_after = get_process_memory();
    println!("API call - Before : {} kB, After : {} kB", mem_before, mem_after);
    HttpResponse::Ok().content_type("text/html").body(contents)
}

// Start the server in main.
#[actix_web::main]
async fn main() -> io::Result<()> {
    // Memory before starting web-server
    let mem_before = get_process_memory();
    println!("Memory Before starting server {} kB", mem_before);
    let server = HttpServer::new(|| { App::new().service(index) }).bind("0.0.0.0:8080")?;
    let mem_after = get_process_memory();
    println!("Memory After starting server {} kB", mem_after);
    server.run().await
}
