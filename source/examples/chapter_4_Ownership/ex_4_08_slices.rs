// Slices:
fn first_word(s: &String) -> usize {
	// Converts the string to bytes
	let bytes = s.as_bytes();

	// Iterate over the converted bytes
	// enumeration returns element as part of a tuple
	// 1st element of tuple is index and
	// 2nd element is a reference to the element

	// i for the index in tuple &
	// &item for a single byte in the tuple as reference is returned
	for (i, &item) in bytes.iter().enumerate() {
		if item == b' '{
			return i;
		}
	}
	return s.len();
}

fn first_word_with_slices(s: &str) -> &str {

	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[0..i];
		}
	}

	return &s[..]
}

fn main(){

	let mut s = String::from("hello world");

	let word = first_word(&s); // word will get the value 5
	println!("word = {}", word);
	s.clear(); // empties the string, making it equal to "" (NULL)
	// word still has the value 5 here, but there's no more string that
	// we could meaningfully use the value 5 with. word is now totally invalid!

	//
	// String Slices
	// -----
	// String slice is a reference part of a string.
	// 
	//
	 let s = String::from("hello world");

	 let hello = &s[0..5]; //[start_index..ending_index]
	 // start_index = first position of the slice
	 // ending_index = last position + 1
	 let world = &s[6..11];

	 println!("hello = {}", hello);
	 println!("world = {}", world);

	 // To start a slice from 0
	 let slice = &s[0..3];
	 println!("slice = {}", slice);
	 let slice = &s[..2]; //no need to write 0
	 println!("slice = {}", slice);

	 // To include the last byte in slice
	 let len = s.len();

	 let slice = &s[3..len];
	 println!("slice = {}", slice);	 
	 let slice = &s[3..]; //no need to add len at end
	 println!("slice = {}", slice);


	 // To take the entire string as slice
	 let len = s.len();

	 let slice = &s[0..len];
	 println!("slice = {}", slice);
	 let slice = &s[..];
	 println!("slice = {}", slice);


	let s = String::from("hello world");

	let word = first_word_with_slices(&s[..]); // word will be a slice of s
	println!("word = {}", word);

	let s_lit = "Hello World";
	let word_lit = first_word_with_slices(&s_lit[..]); // word will be a slice of s
	println!("word = {}", word_lit);

	let my_string = String::from("hello world");
	// `first_word_with_slices` works on slices of `String`s, whether partial
	// or whole.
	let word = first_word_with_slices(&my_string[0..6]);
	println!("word = {}", word);
	let word = first_word_with_slices(&my_string[..]);
	println!("word = {}", word);
	// `first_word_with_slices` also works on references to `String`s, which
	// are equivalent to whole slices of `String`s.
	let word = first_word_with_slices(&my_string);
	println!("word = {}", word);
	let my_string_literal = "hello world";
	// `first_word_with_slices` works on slices of string literals,
	// whether partial or whole.
	let word = first_word_with_slices(&my_string_literal[0..6]);
	println!("word = {}", word);
	let word = first_word_with_slices(&my_string_literal[..]);
	println!("word = {}", word);
	// Because string literals *are* string slices already,
	// this works too, without the slice syntax!
	let word = first_word_with_slices(my_string_literal);
	println!("word = {}", word);


	/* Other Slices */
	let a = [1, 2, 3, 4, 5];
	let slice = &a[1..3];

	assert_eq!(slice,&[2, 3]);
	println!("slice {:?}",slice);
}