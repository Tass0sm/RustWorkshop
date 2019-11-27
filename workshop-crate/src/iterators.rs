fn iterators() {

    let scores = vec![100, 90, 85];

    for score in &scores {
        println!("score: {}", score);
    }

    for score in scores {
        println!("score: {}", score);
    }

    //for i in 0..10 {
    //    println!("{}", i);
    //}
    //
    //for i in (0..=10).map(|x| x * 2) {
    //    println!("{}", i);
    //}

    for i in (0..10).filter(|y| y % 2 == 0) {
        println!("{}", i);
    }

    let values: Vec<i32> = (0..10).collect();

    let sum: i32 = (0..10).sum();
}

