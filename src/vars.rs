// Variables hold primitive data or references to data
//  Variables are immutable by default
// Rust is a block-scoped language

pub fn run()
{
    let name = "Elton";
    let mut age = 42;
    println!("My name is {}, and I am {}.", name, age);
    age = 43;

    println!("My name is {}, and I am {}.", name, age);


    // Defince constant
        const ID: i32 = 001;
        println!("ID: {}", ID);

        //Assign multiple vars
        let (my_name, my_age) = ("Brad", 37);
        println!("{} is {}.", my_name, my_age);
}
