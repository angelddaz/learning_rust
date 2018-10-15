fn main() { 
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
    // mutable datatypes may change size and therefore are allocated on the heap instead of the stack
}
