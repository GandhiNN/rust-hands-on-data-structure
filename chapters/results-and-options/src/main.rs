fn main() {
    let a = divide(10, 5);

    let b = divide(10, 0);

    if let Result::Ok(v) = a {
        println!("val = {:?}", v)
    }

    println!("a = {:?}, b = {:?}", a, b);
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Result::Err("Cannot divide by zero".to_string());
    }
    Result::Ok(a / b)
}
