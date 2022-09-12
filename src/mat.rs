use ndarray::{arr1, arr2};

pub fn run()
{
println!("\n\n\n\n\n\n\n");

let a = arr2(&[[1, 2, 3],
    [4, 5, 6]]);

let b = arr2(&[[6, 5, 4],
    [3, 2, 1]]);

    let c = arr2(&[[6, 3],
        [5, 2],
        [4, 1]]);
let sum = &a + &b;
let scalar = 4;
let vector = arr1(&[1, 2, 3]);

println!("A:\n {}", a);
println!("B:\n {}", b);
println!("C:\n {}", c);
println!("The scalar: {}\n", scalar);
println!("The vector v:\n {}", vector);
println!("\n\nThe matrix sum, A + B = \n {}", sum);
println!("The matrix multiplication, A * C =\n {}", a.dot(&c));
println!("The scalar multiple, 4v is: {}", scalar * vector);
}