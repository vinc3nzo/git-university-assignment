fn fib(n: u32) -> u128 {
    if n == 0 { panic!("shouldn't have done that") }
    if n == 1 { 0 }
    else if n == 2 { 1 }
    else { fib(n - 1) + fib(n - 2) }
}

fn main() {
    println!("{}", fib(10));
}