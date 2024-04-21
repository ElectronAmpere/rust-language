#[derive(Debug)] // Attribute to enable debug trait
struct Rectangle {
	width:u32,
	height:u32,
}

fn main() {
	let rect = Rectangle {
		width: 10,
		height: 20,
	};

	let scale = 2;
	let rect1 = Rectangle {
		width: dbg!(30 * scale), //returns ownership of the value
		height: 54,
	};

	println!("Rect = {:?}", rect);
	dbg!(&rect1);
}
