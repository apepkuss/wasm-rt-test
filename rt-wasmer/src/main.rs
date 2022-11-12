use std::time::Instant;
use wasmer::{Instance, Module, Store};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_engine_universal::Universal;
use wasmer_wasi::WasiState;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();

    // Let's declare the Wasm module with the text representation.
    let curr_dir = std::env::current_dir()?;
    let wasm_bytes = std::fs::read(curr_dir.join("target/wasm32-wasi/release/test-app.wasm"))?;

    // Create a Store.
    // Note that we don't need to specify the engine/compiler if we want to use
    // the default provided by Wasmer.
    // You can use `Store::default()` for that.
    let store = Store::new(&Universal::new(Cranelift::default()).engine());
    println!(
        "time cost create store: {:?} us",
        start.elapsed().as_micros()
    );
    // Let's compile the Wasm module.
    let module = Module::new(&store, wasm_bytes)?;
    println!(
        "time cost compile module: {:?} us",
        start.elapsed().as_micros()
    );

    // First, we create the `WasiEnv`
    let mut wasi_env = WasiState::new("a").finalize()?;
    println!(
        "time cost creating WasiEnv: {:?} us",
        start.elapsed().as_micros()
    );

    // Then, we get the import object related to our WASI
    // and attach it to the Wasm instance.
    let import_object = wasi_env.import_object(&module)?;
    let instance = Instance::new(&module, &import_object)?;

    println!(
        "time cost load module: {:?} us",
        start.elapsed().as_micros()
    );

    let fib = instance.exports.get_function("_start")?;
    fib.call(&[])?;
    println!(
        "time cost call 1000*10000 times fib(30): {:?} us",
        start.elapsed().as_micros()
    );

    Ok(())
}

// #[test]
// #[cfg(feature = "wasi")]
// fn test_wasi() -> Result<(), Box<dyn std::error::Error>> {
//     main()
// }
