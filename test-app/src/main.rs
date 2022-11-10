#[no_mangle]
pub fn fib(n: usize) -> usize {
    let mut a = 1;
    let mut b = 1;

    for _ in 1..n {
        let old = a;
        a = b;
        b += old;
        // println!("round {}: fib({}) = {}", i, i, b);
    }
    // println!("fib({}) = {}", b, b);
    b
}

fn main() {
    for _ in 0..1000 {
        for _ in 0..10000 {
            fib(30);
        }
    }
}
