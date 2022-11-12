use std::time::Instant;
use wasmedge_sdk::{
    config::{
        CommonConfigOptions, CompilerConfigOptions, ConfigBuilder, HostRegistrationConfigOptions,
    },
    params, Vm,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    #[cfg(target_os = "linux")]
    let aot_file = "aot-test-app.so";
    #[cfg(target_os = "macos")]
    let aot_file = "aot-test-app.dylib";

    println!(
        "time cost intialize config: {:?} us",
        start.elapsed().as_micros()
    );

    let vm = Vm::new(Some(config))?;

    println!("time cost create vm: {:?} us", start.elapsed().as_micros());

    let _ = vm
        .run_wasm_from_file_async(aot_file, "_start", params!())
        .await?;

    println!(
        "time cost call 1000*10000 times fib(30): {:?} us",
        start.elapsed().as_micros()
    );
    Ok(())
}
