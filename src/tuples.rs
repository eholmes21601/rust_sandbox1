// Tuples group together values of different typles
//  Max 12 elements



pub fn run()
{

        let person: (&str, &str, i8) = ("Elton", "Maryland", 42);
        println!("{} is from {} and is {}.", person.0, person.1, person.2);

        let (age, name) = (31, "Piseth");
        println!("{} is {} years old.", name, age );


}