// Ownership in Rust

fn main()
{
    { // s is not valid here, since it's not declared yet.
	let s = "Hello";

	println!("Print s = {}", s); // s is valid from this point forward
    } // this scope is now over, and s is no longer valid
    
    {
	let mut s = String::from("Hello");
	s.push_str(", World!"); // push_str() appends a literal to a string.
	println!("{s}");
    }
    {
	let s1 = String::from("hello");
	let s2 = s1.clone();
	println!("s1 = {s1}, s2 = {s2}");
    }

    //Rust example starts
    {
	let s = String::from("Hello"); // s comes into scope

	takes_ownership(s); // s value is moved into the function...
	// ... and so the s is no longer valid here

	let x = 5; // x comes into scope

	makes_copy(x); // x would move into the function, but i32 is a Copy, so
	// it's okay to use x afterward
    } // here x goes out of scope, then s. However, because s value was moved.
}

fn takes_ownership(some_string: String)
{ // some_string comes into scope
    println!("{some_string}");
} // here, some_string goes out of scope and 'drop' is called.
// The backing memory is freed.

fn makes_copy(some_integer: i32)
{// some_integer comes into scope
    println!("{some_integer}");
} // here, some_integer goes out of scope. Nothing special happens.
