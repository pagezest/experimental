pub mod memory;
use memory::get_process_memory;
use std::fs;
use tiny_http::{ Server, Response };

fn main() {
    let mem_before = get_process_memory();
    let server = Server::http("0.0.0.0:8080").unwrap();
    let mem_after = get_process_memory();
    println!("Memory Before starting server {} kB", mem_before);
    println!("Memory After starting server {} kB", mem_after);
    for request in server.incoming_requests() {
        let path = request.url();
        match path {
            "/" => {
                let mem_before = get_process_memory();
                let contents = match fs::read_to_string("index.html") {
                    Ok(data) => data,
                    Err(_) => "Error loading file".to_string(),
                };
                let response = Response::from_string(contents).with_header(
                    tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap()
                );
                request.respond(response).unwrap();
                let mem_after = get_process_memory();
                println!("API call - Before : {} kB, After : {} kB", mem_before, mem_after);
            }
            _ => {
                let response = Response::from_string("404 Not Found").with_status_code(404);
                request.respond(response).unwrap();
            }
        }
    }
}
