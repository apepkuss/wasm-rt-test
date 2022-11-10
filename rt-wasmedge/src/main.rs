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
        "time cost create result: {:?} ms",
        start.elapsed().as_millis()
    );

    // create a Config context
    let config = ConfigBuilder::new(CommonConfigOptions::new().bulk_memory_operations(true))
        .with_compiler_config(CompilerConfigOptions::new().interruptible(true))
        .with_host_registration_config(HostRegistrationConfigOptions::default().wasi(true))
        .build()?;

    let file = "/Users/sam/workspace/rust/benchmark/target/wasm32-wasi/release/test-app.wasm";

    println!(
        "time cost intialize config: {:?} ms",
        start.elapsed().as_millis()
    );

    let vm = Vm::new(Some(config))?;

    println!("time cost create vm: {:?} ms", start.elapsed().as_millis());

    let _ = vm.run_func_from_file(file, "_start", params!())?;

    println!(
        "time cost call 1000*10000 times fib(30): {:?} ms",
        start.elapsed().as_millis()
    );
    Ok(())
}
