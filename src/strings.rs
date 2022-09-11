/*  Primitive str = Immutable fixed-length string somewhere in memory
String = Growable, heap-allocated data structure.  Use when you need to modify or own
string data */

pub fn run()
{
    let mut hello = String::from("Hello ");

    // Get length
    println!("Length: {}", hello.len());
    println!("{}", hello);
    // Push one character
    hello.push('\u{1F4A9}');
    //Push more than one character
    println!("{}", hello);
    hello.push_str(" silly!");
    println!("{}", hello);
    println!("Length: {}", hello.len());
    println!("Is empty?: {}", hello.is_empty());
    println!("Contains Hell?: {}", hello.contains("Hell"));
    println!("Replace: {}", hello.replace("Hello", "Junk"));

    for token in hello.split_whitespace()
    {
        println!("{}", token);
    }

    //Create string with a capacity
    let mut s = String::with_capacity(10);
    s.push('x');
    s.push('y');
    println!("{}", s);

    // Assertion testing
    assert_eq!(2, s.len());
    assert_ne!(2, s.len());
}