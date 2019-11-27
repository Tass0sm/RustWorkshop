fn fibonacci(n: i32) -> i32 {
    if n < 2 {
        n
    } else {
        fibonacci(n-1) + fibonacci(n-2)
    }
}

fn another_fibonacci(n: i32) -> i32 {
    if n < 2 {
        return n;
    }
    return another_fibonacci(n-1) + another_fibonacci(n-2);
}

fn third_fibonacci(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => third_fibonacci(n-1) + third_fibonacci(n-2)
    }
}
