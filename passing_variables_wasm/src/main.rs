mod memory;
use crate::memory::get_process_memory;

use std::error::Error;
use wasmi::{ Caller, Config, Engine, Func, Linker, Module, Store, TypedFunc };

fn main() -> Result<(), Box<dyn Error>> {
    let m1 = get_process_memory();
    for _ in 0..25 {
        let mem_before = get_process_memory();
        let _ = call_wasm();
        let mem_after = get_process_memory();
        println!("Wasm Overhead {} kB", mem_after.saturating_sub(mem_before));
    }
    let m2 = get_process_memory();
    println!("============================");
    println!("Memory at Start : {} kB", m1);
    println!("Memory at End : {} kB", m2);
    Ok(())
}

fn call_wasm() -> Result<(), Box<dyn Error>> {
    let wasm = std::fs::read("debug.wasm")?;
    let engine = Engine::new(&Config::default());
    let module = Module::new(&engine, &wasm)?;

    let mut store = Store::new(&engine, ());
    let mut linker = Linker::new(&engine);
    let abort_func = Func::wrap(&mut store, abort_stub);
    linker.define("env", "abort", abort_func)?;

    let instance = linker.instantiate(&mut store, &module)?.start(&mut store)?;

    let memory = instance
        .get_export(&store, "memory")
        .and_then(|ext| ext.into_memory())
        .unwrap();

    // Loading greet function and passing input as string and receiving output as string.
    let greet: TypedFunc<(u32, u32), u32> = instance.get_typed_func(&store, "greet").unwrap();
    let input = br#"ABCD-1234"#;
    let offset = 0u32;
    memory.write(&mut store, offset as usize, input).unwrap();

    let res_ptr = greet.call(&mut store, (offset, input.len() as u32)).unwrap();
    let mut output = Vec::new();
    let mut curr_ptr = res_ptr;
    loop {
        let mut buf = [0u8, 1];
        memory.read(&mut store, curr_ptr as usize, &mut buf).unwrap();
        if buf[0] == 0 {
            break;
        }
        output.push(buf[0]);
        curr_ptr += 1;
    }

    let greeting = String::from_utf8(output).unwrap();
    assert!(greeting == format!("Hello {}", String::from_utf8_lossy(input)));

    // Loading join_name function and passing input as JSON string and receiving output as string.
    let join_name: TypedFunc<(u32, u32), u32> = instance
        .get_typed_func(&store, "join_name")
        .unwrap();

    let json_input = br#"{"first_name":"H","last_name":"J"}"#;
    memory.write(&mut store, offset as usize, json_input).unwrap();
    let ans_ptr = join_name.call(&mut store, (offset, json_input.len() as u32)).unwrap();

    let mut output_bytes = Vec::new();
    let mut current_ptr = ans_ptr;
    loop {
        let mut buff = [0u8, 1];
        memory.read(&mut store, current_ptr as usize, &mut buff)?;

        if buff[0] == 0 {
            break;
        }
        output_bytes.push(buff[0]);
        current_ptr += 1;
    }

    let full_name = String::from_utf8(output_bytes)?;
    assert!(full_name == "H J");
    Ok(())
}
fn abort_stub(_caller: Caller<'_, ()>, _msg_ptr: i32, _file_ptr: i32, _line: i32, _col: i32) {}
