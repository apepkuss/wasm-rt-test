use anyhow::Error;
use std::time::Instant;
use wasmtime::{Config, Engine, Linker, Module, Store};
// For this example we want to use the async version of wasmtime_wasi.
// Notably, this version of wasi uses a scheduler that will async yield
// when sleeping in `poll_oneoff`.
use wasmtime_wasi::tokio::WasiCtxBuilder;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let start = Instant::now();

    let mut config = Config::new();
    // We need this engine's `Store`s to be async, and consume fuel, so
    // that they can co-operatively yield during execution.
    config.async_support(true);
    config.consume_fuel(true);

    let engine = Engine::new(&config)?;

    let module = Module::from_file(
        &engine,
        "/Users/sam/workspace/rust/benchmark/target/wasm32-wasi/release/test-app.wasm",
    )?;

    // A `Linker` is shared in the environment amongst all stores, and this
    // linker is used to instantiate the `module` above. This example only
    // adds WASI functions to the linker, notably the async versions built
    // on tokio.
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::tokio::add_to_linker(&mut linker, |cx| cx)?;

    println!(
        "time cost create linker: {:?} us",
        start.elapsed().as_micros()
    );
    let wasi = WasiCtxBuilder::new()
        // Let wasi print to this process's stdout.
        .inherit_stdout()
        .build();

    println!("time cost build wasi: {:?} us", start.elapsed().as_micros());
    let mut store = Store::new(&engine, wasi);

    println!(
        "time cost create store: {:?} us",
        start.elapsed().as_micros()
    );
    // WebAssembly execution will be paused for an async yield every time it
    // consumes 10000 fuel. Fuel will be refilled u64::MAX times.
    store.out_of_fuel_async_yield(u64::MAX, 10000);

    // Instantiate into our own unique store using the shared linker, afterwards
    // acquiring the `_start` function for the module and executing it.
    let instance = linker.instantiate_async(&mut store, &module).await?;

    println!(
        "time cost intialize instance: {:?} us",
        start.elapsed().as_micros()
    );

    instance
        .get_typed_func::<(), (), _>(&mut store, "_start")?
        .call_async(&mut store, ())
        .await?;

    println!(
        "time cost call 1000*10000 times fib(30): {:?} us",
        start.elapsed().as_micros()
    );
    Ok(())
}
