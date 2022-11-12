use wasmedge_sys::{Compiler, Config};
use wasmedge_types::CompilerOutputFormat;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let curr_dir = std::env::current_dir()?;
    let wasm_file = curr_dir.join("target/wasm32-wasi/release/test-app.wasm");

    #[cfg(target_os = "linux")]
    let aot_file = std::path::PathBuf::from("aot-test-app.so");
    #[cfg(target_os = "macos")]
    let aot_file = std::path::PathBuf::from("aot-test-app.dylib");

    let mut config = Config::create()?;
    config.set_aot_compiler_output_format(CompilerOutputFormat::Native);

    // create a AOT Compiler with a given configuration
    let compiler = Compiler::create(Some(config))?;

    // compile aot wasm file
    compiler.compile_from_file(wasm_file, &aot_file)?;
    assert!(aot_file.exists());

    Ok(())
}
