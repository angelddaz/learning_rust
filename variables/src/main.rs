fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The vaule of x is: {}", x);

    let _t = true;
    let _f: bool = false; // with explicit type annotation

    // arrays: once declared they cannot grow or shrink in size
    let a = [1, 2, 3, 4, 5];
    // to access array elements
    let _var = a[0];

    another_function();
    another_function2(4);
    println!("{}", five());

    //page 50 and 51 for if else syntax.
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn another_function() {
    println!("Another function.");
}

fn another_function2(x : i32) {
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}