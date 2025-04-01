// Initiating DB - rusqlite.
// Initiating Web Server - tinyhttp.
// Initiating Wasm Module - WAMR.
mod memory;
use memory::get_process_memory;

use rusqlite::{ Connection, Result as RusqliteResult };
use eyre::Result;

use url::Url;
use wamr_rust_sdk::{
    runtime::Runtime,
    module::Module as WamrModule,
    instance::Instance,
    function::Function,
    value::WasmValue,
};

use std::path::PathBuf;
use tiny_http::{ Request, Response, Server };

fn main() -> Result<()> {
    let m1 = get_process_memory();
    let conn = Connection::open_in_memory()?;
    let m2 = get_process_memory();
    init_db(&conn)?;
    let m3 = get_process_memory();
    println!("Start : {} kB", m1);
    println!("After DB Connection {} kB", m2);
    println!("After creating a table {} kB", m3);
    start_server(conn);
    Ok(())
}

fn init_db(conn: &Connection) -> RusqliteResult<()> {
    conn.execute(
        "CREATE TABLE numbers (
            name  TEXT NOT NULL,
            value  INTEGER
        )",
        ()
    )?;
    Ok(())
}

fn store_values_in_db(conn: &Connection, a: i32, b: i32) -> RusqliteResult<()> {
    conn.execute("DELETE FROM numbers", ())?;
    conn.execute("INSERT INTO numbers (name, value) VALUES (?1, ?2)", ("a", a))?;
    conn.execute("INSERT INTO numbers (name, value) VALUES (?1, ?2)", ("b", b))?;
    Ok(())
}

fn fetch_values_from_db(conn: &Connection) -> RusqliteResult<Vec<i32>> {
    let mut stmt = conn.prepare("SELECT value FROM numbers")?;
    let rows = stmt.query_map([], |row| row.get::<_, i32>(0))?;
    let mut nums = Vec::new();
    for num in rows {
        nums.push(num?);
    }
    Ok(nums)
}
fn call_wamr_runtime(a: i32, b: i32) -> Result<i32> {
    let m7 = get_process_memory();
    let runtime = Runtime::new().map_err(|e| eyre::eyre!(e))?;
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("debug.wasm");

    let module = WamrModule::from_file(&runtime, d.as_path()).map_err(|e| eyre::eyre!(e))?;
    let instance = Instance::new(&runtime, &module, 1024 * 64).map_err(|e| eyre::eyre!(e))?;
    let function = Function::find_export_func(&instance, "add").map_err(|e| eyre::eyre!(e))?;

    let params: Vec<WasmValue> = vec![WasmValue::I32(a), WasmValue::I32(b)];
    let result = function.call(&instance, &params).map_err(|e| eyre::eyre!(e))?;

    let m8 = get_process_memory();

    println!(
        "Before WASM module instantiate : {} kB, After WASM module instantiate : {} kB",
        m7,
        m8
    );
    if let Some(WasmValue::I32(decoded)) = result.get(0) {
        Ok(*decoded)
    } else {
        Err(eyre::eyre!("Expected I32 result but got something else"))
    }
}

fn start_server(conn: Connection) {
    let server = Server::http("0.0.0.0:8080").unwrap();
    let m4 = get_process_memory();
    println!("At start of server : {} kB", m4);
    for req in server.incoming_requests() {
        handle_request(&conn, req);
    }
}

fn handle_request(conn: &Connection, req: Request) {
    let m5 = get_process_memory();
    let url = req.url();
    let parsed_url = Url::parse(&format!("http://localhost{}", url)).unwrap();
    let params: std::collections::HashMap<_, _> = parsed_url.query_pairs().collect();

    if let (Some(a), Some(b)) = (params.get("a"), params.get("b")) {
        store_values_in_db(conn, a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()).unwrap();
        let nums = fetch_values_from_db(conn).unwrap();
        let res = call_wamr_runtime(nums[0], nums[1]).unwrap();
        let resp = Response::from_string(format!("The sum of {:#?} is {}", nums, res));
        req.respond(resp).unwrap();
    } else {
        let resp = Response::from_string(
            "Invalid Query Params. Example Use : ?a=10&b=5"
        ).with_status_code(400);
        req.respond(resp).unwrap();
    }

    let m6 = get_process_memory();
    println!("API Call | start {} kB, after - {} kB", m5, m6);
}
