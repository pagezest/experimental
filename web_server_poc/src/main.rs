extern crate may_minihttp;

use std::{ fs, io, thread, time };
use may_minihttp::{ HttpServer, HttpService, Request, Response };
use psutil::process::Process;

fn get_process_memory() -> u64 {
    let pid = std::process::id();
    let process = Process::new(pid).unwrap();
    process.memory_info().unwrap().rss() / 1024
}

#[derive(Clone)]
struct HelloWorld;

impl HttpService for HelloWorld {
    fn call(&mut self, req: Request, res: &mut Response) -> io::Result<()> {
        // Memory while making API call
        let mem_before = get_process_memory();
        match req.path() {
            "/" => {
                let contents = fs::read("index.html")?;
                let contents_string = String::from_utf8(contents).unwrap();
                res.body(Box::leak(contents_string.into_boxed_str()));
            }
            _ => {
                res.status_code(404, "Not Found");
                res.body("404 - Page not found.");
            }
        }
        let mem_after = get_process_memory();
        println!("API call - Before : {} kB, After : {} kB", mem_before, mem_after);
        Ok(())
    }
}

// Start the server in main.
fn main() {
    // Memory before starting web-server
    let mem_before = get_process_memory();
    println!("Memory Before starting server {} kB", mem_before);

    let handle = thread::spawn(|| {
        let server = HttpServer(HelloWorld).start("0.0.0.0:8080").unwrap();
        server.join().unwrap();
    });
    let mem_after = get_process_memory();
    println!("Memory After starting server {} kB", mem_after);
    thread::sleep(time::Duration::new(1, 0));
    let mem_after_5s = get_process_memory();
    println!("Memory After 1 second of starting server {} kB", mem_after_5s);
    handle.join().unwrap();
}
