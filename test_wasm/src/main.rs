use eyre::Result;
use tinywasm::{ Module, Store };

use psutil::process::Process;

fn get_process_memory() -> u64 {
    let pid = std::process::id();
    let process = Process::new(pid).unwrap();
    process.memory_info().unwrap().rss() / 1024 // Convert bytes to KB
}

fn main() -> Result<()> {
    // open the file add.wasm and read it into a Vec<u8>
    for _ in 1..10 {
        let mem_before = get_process_memory();
        println!("Memory before Wasm: {} KB", mem_before);
        call_wasm()?;
        let mem_after = get_process_memory();
        println!("Memory after Wasm: {} KB", mem_after);
        println!("Wasm Overhead: {} KB", mem_after.saturating_sub(mem_before));
    }
    Ok(())
}

fn call_wasm() -> Result<()> {
    let wasm_add = std::fs::read("debug.wasm")?;

    let add_module = Module::parse_bytes(&wasm_add)?;

    let mut store = Store::default();

    // Instantiate the `add` module.
    let add_instance = add_module.instantiate(&mut store, None)?;

    // Call the `main` function, which uses the imported `add` function.
    let main = add_instance.exported_func::<(i32, i32), i32>(&store, "add")?;
    let res = main.call(&mut store, (10, 20))?; // 10 + 20 = 30
    println!("Result: {}", res);

    Ok(())
}
