/*
Primitive Types --
Integers: 48, 18, u16,j116, u32, 132, u64, 164, u128, 1128 (number of bits they take in
memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

/*  Rust is a statically-typed lanauge, which means that it must know the types of all
variables at compile time, however, the compiler can usually infer what type we want to use
based on the value and how we use it. */

pub fn run()
{
    // To make an assignment but not use it, prefix with an "_" to avoid a warning
    let x = 1;
    let y = 2.5;
    let _z: i64 = 43334343434334;

    //Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i32: {}", std::i64::MAX);

    // Boolean
    let is_active = true;
   // Get boolean from expression
   let is_greater =  x < 5;

   //Char
   let a1 = 'a';
   let face = '\u{1F600}';
    println!("{:?}", (x, y, is_active, is_greater, a1, face));

 

}