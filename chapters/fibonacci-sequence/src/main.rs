fn fibonacci_loop(n: i32) {
    let mut start = 0;
    let mut base = 1;

    println!("{}", start);
    println!("{}", base);
    for _ in 0..n - 2 {
        let new_fib = base + start;
        println!("{}", new_fib);
        start = base;
        base = new_fib;
    }
}

fn fibonacci_recursion(n: i32) -> u64 {
    if n < 0 {
        panic!("{} is negative", n);
    }
    match n {
        0 => panic!("zero is not a right argument to fibonacci_recursive()!"),
        1 | 2 => 1,
        _ => fibonacci_recursion(n - 1) + fibonacci_recursion(n - 2),
    }
}

fn find_fibonacci_number(n: i32) -> u64 {
    if n <= 0 {
        panic!("{} is negative", n);
    }
    match n {
        1 | 2 => 1,
        _ => find_fibonacci_number(n - 1) + find_fibonacci_number(n - 2),
    }
}

fn main() {
    let n = 20;
    println!("Fibonacci loop");
    fibonacci_loop(n);

    println!("Fibonacci recursion");
    for i in 1..n {
        let res = fibonacci_recursion(i);
        println!("{}", res);
    }

    println!("Find Fibonacci number");
    let ans = find_fibonacci_number(21);
    println!("{}", ans);
}
