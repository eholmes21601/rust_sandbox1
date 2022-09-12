use ndarray::{arr1, arr2, Array1};

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
// Matrix Addition
println!("\n\nThe matrix sum, A + B = \n {}", sum);
// Matrix Multiplication
println!("The matrix multiplication, A * C =\n {}", a.dot(&c));
// Scalar Multiplication
let new_vector: Array1<_> = scalar * vector;
println!("The scalar multiple, 4v is: {}\n", new_vector);
// Multiple Matrix Operations
println!("The A * 4v is: {}\n", a.dot(&new_vector));
}