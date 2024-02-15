
// To print the fibonacci series
use std::io;

fn main()
{
    let (mut a, mut b) = (0, 1);
    let mut c;

    println!("Enter the range of fibonacci:");

    let mut n = String::new();

    // Read line
    io::stdin().read_line(&mut n).expect("Failed to read line");

    let n: u32 = match n.trim().parse() {
	Ok(num) => num,
	Err(_) => 0,
    };

   // println!("The range is: {n}");

    print!("{a} {b} ");
    
    for _i in 0..n {
	c = a + b;
	print!("{c} ");
	a = b;
	b = c;
    }
    println!("");
}
