fn main() {
    //println!("Hello, world!");
    // let mut age = 21;
    // age += 1 ;
    // println!("{}", age);

    // let input = 10;
    // println!("{} -> {}", input, fibonacci(input));

    let mut scores = vec![100, 90, 85];
    scores[0] -= 10;
    scores.push(100);
    // println!("scores: {:?}", scores);
    for scores in &scores {
        println!("score: {}", scores );
    }

// fn fibonacci(n : u32) -> u32 {
//     if n == 0 {
//         0
//     } else if n == 1 {
//         1
//     } else {
//         fibonacci(n - 1) + fibonacci(n - 2)
//     }
}