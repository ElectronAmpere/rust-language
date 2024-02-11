// Control Flow in Rust
fn main()
{
    let number = 3;

    if number < 3 {
	println!("Condition is true"); // statements or arms
    } else {
	println!("Condition is false");
    }

    // this will return error
    //if number { 
	println!("Number is {number}");
    //}

    if number != 0 {
	println!("Number is {number}");
    }

    multiple_ifs();
    if_as_statement();
    //if_as_statement_mismatch();
    //repetition();
    repetition_with_result();
    loop_labels();

}


fn multiple_ifs()
{
    let number = 6;

    if number % 4 == 0 {
	println!("Divisible by 4");
    } else if number % 3 == 0 {
	println!("Divisible by 3");
    } else if number % 2 == 0 {
	println!("Divisible by 2");
    } else {
	println!("number is not divisible by 4, 3, or 2");
    }
}

fn if_as_statement()
{
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is {number}");
}

fn if_as_statement_mismatch()
{
    let _condition = true;
    //let number = if condition { 5 } else { "six" }; // will generate error

    //println!("The value of number is {number}");
}


fn repetition()
{
    loop { // indefinite loop
	println!("again");
    }
}

fn repetition_with_result()
{
    let mut counter = 0;

    let result = loop {
	counter += 1;

	if counter == 10 {
	    break counter * 2;
	}
    };

    println!("The result is {result}");
}

fn loop_labels()
{
    let mut count = 0;
    'counting_up: loop {
	println!("Count = {count}");

	let mut remaining = 10;

	loop {
	    println!("Remaining = {remaining}");

	    if remaining == 9 {
		break;
	    }
	    if count == 2 {
		break 'counting_up;
	    }
	    remaining -= 1;
	}
	
	count += 1;
    }

    println!("End Count = {count}");
}
