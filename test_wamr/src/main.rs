use wamr_rust_sdk::{
    runtime::Runtime,
    module::Module,
    instance::Instance,
    function::Function,
    value::WasmValue,
    RuntimeError,
};
use std::path::PathBuf;
use psutil::process::Process;

fn get_process_memory() -> u64 {
    let pid = std::process::id();
    let process = Process::new(pid).unwrap();
    process.memory_info().unwrap().rss() / 1024 // Convert bytes to KB
}

fn main() -> Result<(), RuntimeError> {
    for _ in 1..10 {
        let mem_before = get_process_memory();
        println!("Memory before Wasm: {} KB", mem_before);
        call_wamr_runtime()?;
        let mem_after = get_process_memory();
        println!("Memory after Wasm: {} KB", mem_after);
        println!("Wasm Overhead: {} KB", mem_after.saturating_sub(mem_before));
    }
    Ok(())
}

fn call_wamr_runtime() -> Result<(), RuntimeError> {
    let runtime = Runtime::new()?;
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("debug.wasm");

    let module = Module::from_file(&runtime, d.as_path())?;
    let instance = Instance::new(&runtime, &module, 1024 * 64)?;
    let function = Function::find_export_func(&instance, "add")?;

    let params: Vec<WasmValue> = vec![WasmValue::I32(9), WasmValue::I32(27)];
    let result = function.call(&instance, &params)?;

    assert_eq!(result[0], WasmValue::I32(36));
    Ok(())
}
