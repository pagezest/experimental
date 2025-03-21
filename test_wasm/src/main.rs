use std::{ env, thread };
use eyre::Result;

use psutil::process::Process;
use std::path::PathBuf;

use tinywasm::{ Module, Store };
use wasmi::{ Engine, Linker, Module as WasmiModule, Store as WasmiStore };
use wamr_rust_sdk::{
    runtime::Runtime,
    module::Module as WamrModule,
    instance::Instance,
    function::Function,
    value::WasmValue,
};

fn get_process_memory() -> u64 {
    let pid = std::process::id();
    let process = Process::new(pid).unwrap();
    process.memory_info().unwrap().rss() / 1024 // Convert bytes to KB
}

#[derive(Debug, Clone, Copy)]
enum WasmVM {
    TinyWASM,
    Wasmi,
    Wamr,
}

impl WasmVM {
    fn from_str(vm: &str) -> Option<Self> {
        match vm {
            "tinywasm" => Some(Self::TinyWASM),
            "wasmi" => Some(Self::Wasmi),
            "wamr" => Some(Self::Wamr),
            _ => None,
        }
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Please add an argument to specify WASM VM.");
        println!("Try : cargo run -- <tinywasm|wasmi> <num_instance>");
        return Ok(());
    }
    let wasm_vm = match WasmVM::from_str(&args[1]) {
        Some(vm) => vm,
        None => {
            eprintln!("Invalid WASM VM.");
            return Ok(());
        }
    };

    let num_instances: usize = args[2].parse().unwrap_or(1);

    println!("Running {} Instances of {:?}", num_instances, wasm_vm);

    let handles: Vec<_> = (0..num_instances)
        .map(|_| {
            thread::spawn(move || {
                let mem_before = get_process_memory();
                //println!("Memory Before {} kB", mem_before);

                (
                    match wasm_vm {
                        WasmVM::TinyWASM => call_tiny_wasm(),
                        WasmVM::Wasmi => call_wasmi_wasm(),
                        WasmVM::Wamr => call_wamr_runtime(),
                    }
                ).unwrap();

                let mem_after = get_process_memory();
                //println!("Memory After {} kB", mem_after);
                println!("Wasm Overhead {} kB", mem_after.saturating_sub(mem_before));
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    Ok(())
}

fn call_tiny_wasm() -> Result<()> {
    let wasm_add = std::fs::read("debug.wasm")?;
    let add_module = Module::parse_bytes(&wasm_add)?;
    let mut store = Store::default();
    let add_instance = add_module.instantiate(&mut store, None)?;
    let main = add_instance.exported_func::<(i32, i32), i32>(&store, "add")?;
    let res = main.call(&mut store, (10, 20))?; // 10 + 20 = 30
    assert_eq!(res, 30);
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
    assert_eq!(res, 30);
    Ok(())
}

fn call_wamr_runtime() -> Result<()> {
    let runtime = Runtime::new().map_err(|e| eyre::eyre!(e))?;
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("debug.wasm");
    let module = WamrModule::from_file(&runtime, d.as_path()).map_err(|e| eyre::eyre!(e))?;
    let instance = Instance::new(&runtime, &module, 1024 * 64).map_err(|e| eyre::eyre!(e))?;
    let function = Function::find_export_func(&instance, "add").map_err(|e| eyre::eyre!(e))?;
    let params: Vec<WasmValue> = vec![WasmValue::I32(10), WasmValue::I32(20)];
    let result = function.call(&instance, &params).map_err(|e| eyre::eyre!(e))?;
    assert_eq!(result[0], WasmValue::I32(30));
    Ok(())
}
