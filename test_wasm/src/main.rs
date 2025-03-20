use std::env;
use eyre::Result;
use tinywasm::{ Module, Store };

use psutil::process::Process;
use wasmi::{ Engine, Linker, Module as WasmiModule, Store as WasmiStore };

fn get_process_memory() -> u64 {
    let pid = std::process::id();
    let process = Process::new(pid).unwrap();
    process.memory_info().unwrap().rss() / 1024 // Convert bytes to KB
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please add an argument to specify WASM VM.");
        println!("Try : cargo run -- <tinywasm|wasmi>");
        return Ok(());
    }
    let wasm_vm = &args[1];
    match wasm_vm.as_str() {
        "tinywasm" => {
            println!("-----TinyWASM-----");
            for _ in 1..10 {
                let mem_before = get_process_memory();
                println!("Memory before Wasm: {} KB", mem_before);
                call_tiny_wasm()?;
                let mem_after = get_process_memory();
                println!("Memory after Wasm: {} KB", mem_after);
                println!("Wasm Overhead: {} KB", mem_after.saturating_sub(mem_before));
            }
        }
        "wasmi" => {
            println!("-----WASMI-----");
            for _ in 1..10 {
                let mem_before = get_process_memory();
                println!("Memory before Wasm: {} KB", mem_before);
                call_wasmi_wasm()?;
                let mem_after = get_process_memory();
                println!("Memory after Wasm: {} KB", mem_after);
                println!("Wasm Overhead: {} KB", mem_after.saturating_sub(mem_before));
            }
        }
        "2-tinywasm" => {
            println!("-----2 Tinywasm VM-----");
            for _ in 1..10 {
                let mem_before = get_process_memory();
                println!("Memory before Wasm: {} KB", mem_before);
                call_tiny_wasm()?;
                let mem_after = get_process_memory();
                println!("Memory after Wasm: {} KB", mem_after);
                println!("Wasm Overhead: {} KB", mem_after.saturating_sub(mem_before));
                call_sub_tinywasm()?;
                let mem_after_sub = get_process_memory();
                println!("Memory after Wasm-2: {} KB", mem_after_sub);
                println!("Wasm Overhead-2: {} KB", mem_after_sub.saturating_sub(mem_after));
            }
        }
        "2-wasmi" => {
            println!("-----2 WASMI VM-----");
            for _ in 1..10 {
                let mem_before = get_process_memory();
                println!("Memory before Wasm: {} KB", mem_before);
                call_wasmi_wasm()?;
                let mem_after = get_process_memory();
                println!("Memory after Wasm: {} KB", mem_after);
                println!("Wasm Overhead: {} KB", mem_after.saturating_sub(mem_before));
                call_sub_wasmi()?;
                let mem_after_sub = get_process_memory();
                println!("Memory after Wasm-2: {} KB", mem_after_sub);
                println!("Wasm Overhead-2: {} KB", mem_after_sub.saturating_sub(mem_after));
            }
        }
        _ => {
            eprintln!("Invalid WasmVM. Use \"tinywasm\" or \"wasmi\"");
        }
    }
    Ok(())
}

fn call_tiny_wasm() -> Result<()> {
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

fn call_wasmi_wasm() -> Result<()> {
    let wasm_add = std::fs::read("debug.wasm")?;

    let engine = Engine::default();
    let module = WasmiModule::new(&engine, wasm_add)?;

    let mut store = WasmiStore::new(&engine, ());

    let linker = Linker::new(&engine);

    let add_instance = linker.instantiate(&mut store, &module)?.start(&mut store)?;
    let add_func = add_instance.get_typed_func::<(i32, i32), i32>(&mut store, "add")?;
    let res = add_func.call(&mut store, (10, 20))?;
    println!("Results : {}", res);

    Ok(())
}

fn call_sub_tinywasm() -> Result<()> {
    let wasm_sub = std::fs::read("sub.wasm")?;
    let add_module = Module::parse_bytes(&wasm_sub)?;
    let mut store = Store::default();
    let add_instance = add_module.instantiate(&mut store, None)?;
    let main = add_instance.exported_func::<(i32, i32), i32>(&store, "sub")?;
    let res = main.call(&mut store, (20, 10))?;
    println!("Result: {}", res);
    Ok(())
}

fn call_sub_wasmi() -> Result<()> {
    let wasm_sub = std::fs::read("sub.wasm")?;
    let engine = Engine::default();
    let module = WasmiModule::new(&engine, wasm_sub)?;
    let mut store = WasmiStore::new(&engine, ());
    let linker = Linker::new(&engine);
    let sub_instance = linker.instantiate(&mut store, &module)?.start(&mut store)?;
    let sub_func = sub_instance.get_typed_func::<(i32, i32), i32>(&mut store, "sub")?;
    let res = sub_func.call(&mut store, (20, 10))?;
    println!("Results : {}", res);
    Ok(())
}
