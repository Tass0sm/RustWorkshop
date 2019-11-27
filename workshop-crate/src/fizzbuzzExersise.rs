fn bad_fizzbuzz_fn(n: i32) {
    if n % 3 == 0 && n % 5 != 0 {
        println!("fizz");
    } else if n % 5 == 0 && n % 3 != 0 {
        println!("buzz");
    } else if n % 5 == 0 && n % 3 == 0 {
        println!("fizzbuzz");
    } else {
        println!("{}", n);
    }
}

fn bad_fizzbuzz_ret_fn(n: i32) -> String {
    if n % 3 == 0 && n % 5 != 0 {
        "fizz".to_string()
    } else if n % 5 == 0 && n % 3 != 0 {
        "buzz".to_string()
    } else if n % 5 == 0 && n % 3 == 0 {
        "fizzbuzz".to_string()
    } else {
        n.to_string()
    }
}

fn fizzbuzz_fn(n: i32) {
    match (n % 3 == 0, n % 5 != 0) {
        (true, false) => println!("fizz"),
        (false, true) => println!("buzz"),
        (true, true)  => println!("fizzbuzz"),
        (false,false) => println!("{}", n)
    }
}


fn main() {

    for i in 0..=100 {
        //fizzbuzz_fn(i);
        //println!("{}", fizzbuzz_ret_fn(i));
    }
}

