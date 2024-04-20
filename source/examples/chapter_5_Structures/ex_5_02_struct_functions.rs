struct User {
	active: bool,
	username: String,
	email: String,
	sign_in_count: u64,
}

fn build_user (email: String, username: String) -> User {
	User {
		active: true,
		username: username,
		email: email,
		sign_in_count: 1,
	}
}

// Uses field init shorthand
fn build_user_shorthand (email: String, username: String) -> User {
	User {
		active: true,
		username,
		email,
		sign_in_count: 1,
	}
}

fn main() {
	let user1 = User {
		active: true,
		username: String::from("dummy"),
		email: String::from("dummy@email.com"),
		sign_in_count: 1,
	};

	// without struct update syntax
	let user2 = User {
		active: user1.active,
		username: user1.username,
		email: String::from("anotherdummy@email.com"),
		sign_in_count: user1.sign_in_count,
	};

	// with struct update syntax
	let _user3 = User {
		email: String::from("anotherdummy1@email.com"),
		..user2 //must be the last one
	};
}