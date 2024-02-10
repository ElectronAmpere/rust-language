// Function in RUST

fn main()
{
    // Statement - don't return a value
    let _x = 6; //statement type1
    //let x = (let y = 6); //statement type2 - generates error
    
    println!("This is a main function");

    another_function(5);
    print_labeled_measurement(5, 'h');
    expression();
    println!("Function Return: {}", return_five());
    println!("Plus One Return: {}", plus_one(return_five()));
}

// Function with parameter
fn another_function(x: i32)
{
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char)
{
    println!("The measurement is: {value}{unit_label}");
}

fn expression()
{
    let y = {
	let x = 3;
	x + 1
    };

    println!("The value of y is: {y}");
}

// Function with return values
// Form 1:
fn return_five() -> i32
{
    5
}

// Form 2:
fn plus_one(x: i32) -> i32
{
    x + 1
}
