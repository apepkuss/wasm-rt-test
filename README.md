# WasmEdge Rust vs. Wasmtime vs. Wasmer

## Test Environment

- OS: macOS 12.3.1
- Hardware: Apple M1 Pro + 32GB RAM
- Rust: 1.67.0-nightly (215e3cd21 2022-11-03)

## Test Programs

- The `test-app` program is from [issue 2073](https://github.com/WasmEdge/WasmEdge/discussions/2073) by @shippomx. It is compiled into the `test-app.wasm` file.

- The source code of `rt-xxx` programs are also derived from [issue 2073](https://github.com/WasmEdge/WasmEdge/discussions/2073) by @shippomx, but some of them are modified to make them more suitable for this test.

- `rt-wasmedge-aot` and `rt-wasmedge-async-aot` are newly introduced in this test.

## Statistics of time cost of 1000*10000 times fib(30)

<img src="statistics.png" alt="statistics" style="zoom:50%;"/>

- The best of 10 samples of `rt-wasmedge`
  
  ```bash
  time cost create result: 0 us
  time cost intialize config: 89 us
  time cost create vm: 238 us
  time cost call 1000*10000 times fib(30): 2036 us
  ```

- The best of 10 samples of `rt-wasmedge-aot`
  
  ```bash
  time cost create result: 0 us
  time cost intialize config: 64 us
  time cost create vm: 198 us
  time cost call 1000*10000 times fib(30): 1665 us
  ```

- The best of 10 samples of `rt-wasmedge-async`
  
  ```bash
  time cost create result: 0 us
  time cost intialize config: 56 us
  time cost create vm: 176 us
  time cost call 1000*10000 times fib(30): 2028 us
  ```

- The best of 10 samples of `rt-wasmedge-async-aot`
  
  ```bash
  time cost create result: 0 us
  time cost intialize config: 54 us
  time cost create vm: 150 us
  time cost call 1000*10000 times fib(30): 1655 us
  ```

- The best of 10 samples of `rt-wasmtime`
  
  ```bash
  time cost create linker: 184376 us
  time cost build wasi: 184703 us
  time cost create store: 184779 us
  time cost intialize instance: 184979 us
  time cost call 1000*10000 times fib(30): 185472 us
  ```

- The best of 10 samples of `rt-wasmer`
  
  ```bash
  time cost create store: 941 us
  time cost compile module: 141817 us
  time cost creating WasiEnv: 142364 us
  time cost load module: 143528 us
  time cost call 1000*10000 times fib(30): 143749 us
  ```

## Test Environment

- OS: Ubuntu-22.04 (on WSL2 on Windows 11)
- Hardware: 11th Gen Intel(R) Core(TM) i7-1165G7 @ 2.80GHz + 16GB RAM
- Rust: 1.67.0-nightly (42325c525 2022-11-11)


- The best of 10 samples of `rt-wasmedge`
  
  ```bash
  time cost create result: 0 us
  time cost intialize config: 39 us
  time cost create vm: 187 us
  time cost call 1000*10000 times fib(30): 2724 us
  ```

- The worst of 10 samples of `rt-wasmedge`
  
  ```bash
  time cost create result: 0 us
  time cost intialize config: 44 us
  time cost create vm: 215 us
  time cost call 1000*10000 times fib(30): 2870 us
  ```

- The best of 10 samples of `rt-wasmedge-aot`
  
  ```bash
  time cost create result: 0 us
  time cost intialize config: 35 us
  time cost create vm: 190 us
  time cost call 1000*10000 times fib(30): 2819 us
  ```

- The worst of 10 samples of `rt-wasmedge-aot`
  
  ```bash
  time cost create result: 0 us
  time cost intialize config: 37 us
  time cost create vm: 194 us
  time cost call 1000*10000 times fib(30): 3124 us
  ```
  
- The best of 10 samples of `rt-wasmedge-async`
  
  ```bash
  time cost create result: 0 us
  time cost intialize config: 17 us
  time cost create vm: 252 us
  time cost call 1000*10000 times fib(30): 2827 us
  ```

- The worst of 10 samples of `rt-wasmedge-async`
  
  ```bash
  time cost create result: 0 us
  time cost intialize config: 31 us
  time cost create vm: 498 us
  time cost call 1000*10000 times fib(30): 3242 us
  ```

- The best of 10 samples of `rt-wasmedge-async-aot`
  
  ```bash
  time cost create result: 0 us
  time cost intialize config: 14 us
  time cost create vm: 255 us
  time cost call 1000*10000 times fib(30): 2951 us
  ```

- The worst of 10 samples of `rt-wasmedge-async-aot`
  
  ```bash
  time cost create result: 0 us
  time cost intialize config: 36 us
  time cost create vm: 595 us
  time cost call 1000*10000 times fib(30): 3376 us
  ```
  
- The best of 10 samples of `rt-wasmtime`
  
  ```bash
  time cost create linker: 665889 us
  time cost build wasi: 665970 us
  time cost create store: 665997 us
  time cost intialize instance: 666124 us
  time cost call 1000*10000 times fib(30): 666272 us
  ```

- The worst of 10 samples of `rt-wasmtime`
  
  ```bash
  time cost create linker: 1032313 us
  time cost build wasi: 1032385 us
  time cost create store: 1032412 us
  time cost intialize instance: 1032541 us
  time cost call 1000*10000 times fib(30): 1032678 us
  ```

- The best of 10 samples of `rt-wasmer`
  
  ```bash
  time cost create store: 736 us
  time cost compile module: 398861 us
  time cost creating WasiEnv: 398964 us
  time cost load module: 399930 us
  time cost call 1000*10000 times fib(30): 400005 us
  ```

- The worst of 10 samples of `rt-wasmer`
  
  ```bash
  time cost create store: 694 us
  time cost compile module: 485110 us
  time cost creating WasiEnv: 485227 us
  time cost load module: 486268 us
  time cost call 1000*10000 times fib(30): 486358 us
  ```
