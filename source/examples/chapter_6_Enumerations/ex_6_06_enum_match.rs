#[derive(Debug)]
enum USstate {
	Alabama,
	Alaska,
}

enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter(USstate),
}

fn value_in_cents(coin: Coin) -> u8 {
	match coin {
		Coin::Penny => {
			println!("Lucky Penny");
			1
		}, // {}, is optional here
		Coin::Nickel => 5,
		Coin::Dime => 10,
		Coin::Quarter(state) => {
			println!("State quarter from {:?}!", state);
			25
		},
	}	
}

fn main() {
	println!("Penny: {}", value_in_cents(Coin::Penny));
	println!("Quarter: {}", value_in_cents(Coin::Quarter(USstate::Alabama)));
}