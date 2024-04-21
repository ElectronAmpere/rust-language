#[derive(Debug)]
struct Rectangle{
	width: u32,
	height: u32,
}

// Implementation of the method area for rectangle
impl Rectangle {
	fn area (&self) -> u32 {
		self.width * self.height
	}
	fn perimeter (self: &Self) -> u32 {
		2 * (self.width + self.height)
	}
	fn width (self: &Self) -> bool {
		self.width > 0
	}
}


fn main() {
	let rect = Rectangle {
		width: 20,
		height: 40,
	};

	println!("Area of the rectangle is {} square pixels", rect.area());
	println!("Perimeter of the rectangle is {} pixels", rect.perimeter());

	if rect.width() {
		println!("The rectangle has a non-zero width, {}", rect.width);
	}
}