fn main() {

    let vec: Vec<i32> = (0..100).filter(|x| x % 3 == 0 && x % 7 == 0).collect();
    let sum: i32 = (0..100).filter(|x| x % 3 == 0 && x % 7 == 0).sum();
    println!("{}", sum);
    
    for i in &vec {
        println!("{}", i);
    }
    
}

