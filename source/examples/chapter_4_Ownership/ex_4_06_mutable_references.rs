// Mutable References in Rust
fn main()
{
    let mut s = String::from("hello");
    {
	let r1 = &mut s;
	let r2 = &mut s;

	println!("{r1}, {r2}");
    }
    {
	{
	    let r1 = &mut s;
	} // r1 goes out of scope here, so we can make a new reference with no problems
	let r2 = &mut s;

	println!("{r1}, {r2}");
    }    
    {
	let r1 = &s; // no problem
	let r2 = &s; // no problem
	let r3 = &mut s; // BIG PROBLEM

	println!("{r1}, {r2}, and {r3}");
    }

    {
	let r1 = &s; // no problem
	let r2 = &s; // no problem
	println!("{r1} and {r2}"); // Variables r1 and r2 will not be used aftee this point.

	let r3 = &mut s; //no problem
	println!("{r3}");
    }
}
