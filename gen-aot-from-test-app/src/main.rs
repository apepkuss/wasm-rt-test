use wasmedge_sys::{Compiler, Config};
use wasmedge_types::CompilerOutputFormat;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wasm_file = "/Users/sam/workspace/rust/benchmark/target/wasm32-wasi/release/test-app.wasm";
    let aot_file = std::path::PathBuf::from("aot-test-app.wasm");

    let mut config = Config::create()?;
    config.set_aot_compiler_output_format(CompilerOutputFormat::Native);

    // create a AOT Compiler with a given configuration
    let compiler = Compiler::create(Some(config))?;

    // compile aot wasm file
    compiler.compile_from_file(wasm_file, &aot_file)?;
    assert!(aot_file.exists());

    Ok(())
}
