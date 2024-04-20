struct User {
	active: bool,
	username: String,
	email: String,
	sign_in_count: u64,
}

fn main() {
	let mut user1 = User {
		active: true,
		username: String::from("dummy"),
		email: String::from("dummy@email.com"),
		sign_in_count: 1,
	};

	user1.email = String::from("anotherdummy@email.com");

	println!("username {:?}",user1.username);
	println!("email {:?}",user1.email);
	println!("active {:?}",user1.active);
	println!("sign_in_count {:?}",user1.sign_in_count);
}