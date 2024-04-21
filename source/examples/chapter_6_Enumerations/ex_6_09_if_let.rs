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

fn main() {
	
	let config_max = Some(3u8);
	
	match config_max {
		Some(max) => println!("The maximum is configured to be {max}"),
		_ => (),
	}

	if let Some(max) = config_max {
		println!("The maximum is configured to be {max}");
	}

	let mut count = 0;

	let coin: Coin = Coin::Penny;
	
	match coin {
		Coin::Quarter(ref state) => println!("State quarter from {:?}!", state),
		_ => count += 1,
	}

	if let Coin::Quarter(ref state) = coin {
		println!("State quarter from {:?}!", state);
	} else {
		count += 1;
	}
}