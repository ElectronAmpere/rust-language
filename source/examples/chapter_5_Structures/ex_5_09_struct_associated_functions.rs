#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	fn square (size: u32) -> Self {
		Self {
			width: size,
			height: size,
		}
	}
}

fn main() {
	let square = Rectangle::square(30);
	println!("Square: (width, height) \n ({},{})", square.width, square.height);
}