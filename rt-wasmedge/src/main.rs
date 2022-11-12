use std::time::Instant;
use wasmedge_sdk::{
    config::{
        CommonConfigOptions, CompilerConfigOptions, ConfigBuilder, HostRegistrationConfigOptions,
    },
    params, Vm,
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    println!(
        "time cost create result: {:?} us",
        start.elapsed().as_micros()
    );

    // create a Config context
    let config = ConfigBuilder::new(CommonConfigOptions::new().bulk_memory_operations(true))
        .with_compiler_config(CompilerConfigOptions::new().interruptible(true))
        .with_host_registration_config(HostRegistrationConfigOptions::default().wasi(true))
        .build()?;

    let curr_dir = std::env::current_dir()?;
    let wasm_file = curr_dir.join("target/wasm32-wasi/release/test-app.wasm");

    println!(
        "time cost intialize config: {:?} us",
        start.elapsed().as_micros()
    );

    let vm = Vm::new(Some(config))?;

    println!("time cost create vm: {:?} us", start.elapsed().as_micros());

    let _ = vm.run_func_from_file(wasm_file, "_start", params!())?;

    println!(
        "time cost call 1000*10000 times fib(30): {:?} us",
        start.elapsed().as_micros()
    );
    Ok(())
}
