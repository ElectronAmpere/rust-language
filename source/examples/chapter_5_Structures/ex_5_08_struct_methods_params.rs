#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

// functions under the impl block are called Associated Functions
impl Rectangle {
	fn _area (&self) -> u32 {
		self.width * self.height
	}

	fn can_hold (&self, other: &Rectangle) -> bool {
		let result: bool;
		result = (self.width > other.width) && (self.height > other.height);
		return result
	}
}

fn main() {
	let rect1 = Rectangle {
		width: 30,
		height: 45,
	};

	let rect2 = Rectangle {
		width: 34,
		height: 45,
	};
	
	let rect3 = Rectangle {
		width: 20,
		height: 35,
	};

	println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
	println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}