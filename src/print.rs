
pub fn run()
{
    //Priting to the console
    println!("Hello from the print.rs file");

    //Basic Formatting
    println!("{} is from {}.", "Elton", "Maryland");

    //Positional Arguments
    println!("{0} is from {1}, and {0} likes to {2}.", "Elton", "Maryland", "code");

    //Named Arguments
    println!("{name} likes to watch {activity}.", name = "Elton", activity = "baseball");

    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10 );

    //Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));
}