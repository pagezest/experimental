// Initiating DB - rusqlite.
// Initiating Web Server - tinyhttp.
// Initiating Wasm Module - WAMR.
mod memory;
use memory::get_process_memory;

use rusqlite::{ Connection, Result as RusqliteResult };
use eyre::Result;

use wamr_rust_sdk::{
    runtime::Runtime,
    module::Module as WamrModule,
    instance::Instance,
    function::Function,
    value::WasmValue,
};

use std::path::PathBuf;

use std::fs;
use tiny_http::{ Server, Response };

fn main() -> Result<()> {
    let m1 = get_process_memory();
    let nums = store_values_in_db()?;
    let m2 = get_process_memory();
    let res = call_wamr_runtime(nums[0], nums[1])?;
    let m3 = get_process_memory();
    println!("Before DB : {} kB", m1);
    println!("After DB : {} kB", m2);
    println!("Instantiate WASM Completed: {} kB", m3);
    start_server(nums, res);
    Ok(())
}

fn store_values_in_db() -> RusqliteResult<Vec<i32>> {
    let conn = Connection::open_in_memory()?;
    conn.execute(
        "CREATE TABLE numbers (
            name  TEXT NOT NULL,
            value  INTEGER
        )",
        ()
    )?;
    conn.execute("INSERT INTO numbers (name, value) VALUES (?1, ?2)", ("a", 10))?;
    conn.execute("INSERT INTO numbers (name, value) VALUES (?1, ?2)", ("b", 20))?;
    let mut stmt = conn.prepare("SELECT value FROM numbers")?;
    let rows = stmt.query_map([], |row| row.get::<_, i32>(0))?;

    let mut nums = Vec::new();
    for num in rows {
        nums.push(num?);
    }
    Ok(nums)
}

fn call_wamr_runtime(a: i32, b: i32) -> Result<i32> {
    let runtime = Runtime::new().map_err(|e| eyre::eyre!(e))?;
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("debug.wasm");

    let module = WamrModule::from_file(&runtime, d.as_path()).map_err(|e| eyre::eyre!(e))?;
    let instance = Instance::new(&runtime, &module, 1024 * 64).map_err(|e| eyre::eyre!(e))?;
    let function = Function::find_export_func(&instance, "add").map_err(|e| eyre::eyre!(e))?;

    let params: Vec<WasmValue> = vec![WasmValue::I32(a), WasmValue::I32(b)];
    let result = function.call(&instance, &params).map_err(|e| eyre::eyre!(e))?;

    if let Some(WasmValue::I32(decoded)) = result.get(0) {
        Ok(*decoded)
    } else {
        Err(eyre::eyre!("Expected I32 result but got something else"))
    }
}


fn start_server(nums: Vec<i32>, res: i32) {
    println!("The sum of {:#?} is {}", nums, res);
    let server = Server::http("0.0.0.0:8080").unwrap();

    for request in server.incoming_requests() {
        let m4 = get_process_memory();
        let path = request.url();
        match path {
            "/" => {
                let template = match fs::read_to_string("index.html") {
                    Ok(data) => data,
                    Err(_) => "Error loading file".to_string(),
                };
                let nums_str = format!("{:?}", nums);
                let contents = template
                    .replace("{{NUMS}}", &nums_str)
                    .replace("{{RESULT}}", &res.to_string());

                let response = Response::from_string(contents).with_header(
                    tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap(),
                );

                request.respond(response).unwrap();
            }
            _ => {
                let response = Response::from_string("404 Not Found").with_status_code(404);
                request.respond(response).unwrap();
            }
        }
        let m5 = get_process_memory();
        println!("API-Call | start {} kB, after - {} kB", m4, m5);
    }
}


