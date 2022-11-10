# WasmEdge Rust vs. Wasmtime vs. Wasmer

## Test Environment

- OS: macOS 12.3.1
- Hardware: Apple M1 Pro + 32GB RAM
- Rust: 1.67.0-nightly (215e3cd21 2022-11-03)

## Statistics of time cost of 1000*10000 times fib(30)

| wasmedge  | wasmedge-async | wasmtime  | wasmer |
| :-------: | :------------: | :-------: | :----: |
| 2 ms      | 2 ms           | 1 ms      | 155 ms |
| 2 ms      | 2 ms           | 1 ms      | 148 ms |
| 2 ms      | 2 ms           | 1 ms      | 151 ms |
| 2 ms      | 2 ms           | 1 ms      | 158 ms |
| 2 ms      | 2 ms           | 1 ms      | 146 ms |
| 2 ms      | 2 ms           | 1 ms      | 152 ms |
| 2 ms      | 2 ms           | 1 ms      | 156 ms |
| 2 ms      | 2 ms           | 1 ms      | 152 ms |
| 2 ms      | 2 ms           | 2 ms      | 146 ms |
| 2 ms      | 2 ms           | 1 ms      | 157 ms |

- The best of 10 samples of `rt-wasmedge`
  
  ```bash
  time cost create result: 0 ms
  time cost intialize config: 0 ms
  time cost create vm: 0 ms
  time cost call 1000*10000 times fib(30): 2 ms
  ```

- The best of 10 samples of `rt-wasmedge-async`
  
  ```bash
  time cost create result: 0 ms
  time cost intialize config: 0 ms
  time cost create vm: 0 ms
  time cost call 1000*10000 times fib(30): 2 ms
  ```

- The Best of 10 samples of `rt-wasmtime`
  
  ```bash
  time cost create linker: 0 ms
  time cost build wasi: 1 ms
  time cost create store: 1 ms
  time cost intialize instance: 1 ms
  time cost call 1000*10000 times fib(30): 1 ms
  ```

- The best of 10 samples of `rt-wasmer`
  
  ```bash
  time cost create store: 0 ms
  time cost compile module: 144 ms
  time cost creating WasiEnv: 144 ms
  time cost load module: 145 ms
  time cost call 1000*10000 times fib(30): 146 ms
  ```
