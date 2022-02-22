fn fib(n: u32) -> u128 {
    if n == 0 { panic!("shouldn't have done that") }
    if n == 1 || n == 2 { return 1 }
    fib(n - 1) + fib(n - 2)
}

fn main() {
    println!("{}", fib(10));
}