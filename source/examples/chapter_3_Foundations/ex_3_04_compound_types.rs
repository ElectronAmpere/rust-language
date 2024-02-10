// Compund Types - Invalid array access
use std::io;

fn main()
{
    let array = [1, 2, 3, 4, 5];

    println!("Enter an array index:");

    let mut index = String::new();

    io::stdin()
	.read_line(&mut index)
	.expect("Unable to read line");

    let index: usize = index
	.trim()
	.parse()
	.expect("Entered index is not a number");

    let element = array[index];

    println!("The value of the element at the index {index} is: {element}");
}
