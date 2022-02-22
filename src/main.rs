fn fib(n: u32) {
    if n == 0 { panic!("shouldn't have done that") }
    if n >= 1 { println!("0"); }
    if n >= 2 { println!("1"); }

    let mut x: u128 = 0;
    let mut y: u128 = 1;
    let mut res: u128;
    for _ in 2..n {
        res = x + y;
        x = y;
        y = res;
        println!("{}", res);
    }
}

fn main() {
    fib(10);
}