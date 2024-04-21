struct Rectangle {
	width:u32,
	height:u32,
}

fn main() {
	let width = 30;
	let height = 40;

	let rect = (width, height);
	let rect1 = Rectangle {
		width,
		height,
	};

	println!("Area of the rectangle is {} square pixels", area(width, height));
	println!("Tuple, Area of the rectangle is {} square pixels", area_tuples(rect));
	println!("Struct, Area of the rectangle is {} square pixels", area_struct(&rect1));
}

fn area(width:u32, height:u32) -> u32 {
 	return height * width
}

fn area_tuples(dimensions: (u32, u32)) -> u32 {
	return dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
 	return rectangle.height * rectangle.width
}