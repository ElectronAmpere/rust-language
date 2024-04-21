fn main() {
	let dice_roll = 9;

	match dice_roll {
		3 => add_fancy_hat(),
		7 => remove_fancy_hat(),
		other => move_player(other), //catch-all pattern arm which uses the value
	}

	match dice_roll {
		3 => add_fancy_hat(),
		7 => remove_fancy_hat(),
		_ => reroll(), // '_' catch-all pattern arm which ignores the value
	}

	match dice_roll {
		3 => add_fancy_hat(),
		7 => remove_fancy_hat(),
		_ => (), // nothing happens here
	}
}

fn add_fancy_hat() {
	println!("Put on a fancy hat!");
}

fn remove_fancy_hat() {
	println!("Take off the fancy hat!");
}

fn reroll() {
	println!("Reroll the dice!");
}

fn move_player(num_spaces:u8) {
	println!("Player moved, {} spaces", num_spaces);
}