fn plus_one(x: Option<i32>) -> Option<i32> {
	match x {
		None => None,
		Some(i) => Some(i + 1), // instead of 'i', 'x' can be used
		// i is binded to value contained in the arm
	}
}


fn main() {
	let five = Some(5);
	let six = plus_one(five);
	let none = plus_one(None);

	dbg!(five);
	dbg!(six);
	dbg!(none);
}