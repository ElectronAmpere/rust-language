use std::io; // This is equivalent to the stdio library in C, under prelude
/* Rng trait defines methods that random number generators implement 
 * and this trait must be in scope for us to use those methods
 */
use rand::Rng;
use std::cmp::Ordering;

fn main() { // fn - function keyword, fn fn_name(){...}
    println!("Guess a number!");

    // Generates a random number within the range 1...100
    // and stores it in the secret number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");
    println!("Please enter your guess.");

    // let - is used to create a variable, let apples = 5;
    // by default the variable are immutable, meaning once 
    // we give the variable a value, the value won’t change
    // 
    // To make the variable mutable, we add mut before the variable name
    // 
    // let apples = 5; //immutable
    // let mut bananas = 5; //mutable
    // 
    // String::new() is a function that returns a new string instance
    // String is a type from standard library that is growable.
    // encoded in UTF-8
    // 
    // The :: in ::new indicates that the new() is an associated function
    // of String type.
    let mut guess = String::new();

    // stdin to read the user input
    // & in read_line indicates that the argument is passed as reference
    // references are also immutable by default hence
    // we write '&mut guess' instead or '&guess'
    // the read_le()in method returns an Result which is an enum
    // the Result also has methods associated with them
    // so, Result.expect("message")
    io::stdin()
        .read_line(&mut guess) // this is a method .method_name()
        .expect("Failed to read line");

    // The guess variable is shadowed to use an u32 value rather than
    // a string as we did previously
    // Also, the .trim() etc.. are methods attached to the original variable guess
    // so this can be expanded into guess.trim().parse().expect();
    // This maybe similar to typecasting in C, kind of
    // Also, the parse() method return ParseIntError when the entered
    // string contains say, `a45d` or `a45`
    let guess: u32 = guess.trim()
                          .parse()
                          .expect("Please type a number!");

    // the {} inside the println! state is used to print the value of the
    // variable
    // 
    // Also, for operating on the variable just before printing it
    // 
    // let y = 5;
    // let x = 10;
    // println!("x = {x} and y + 2 = {}", y + 2);
    // 
    // will result in
    // x = 10 and y + 2 = 7
    println!("You guessed: {guess}");

    /*
     * The `match` expression is made up of arms
     * The arms consists of patterns to match agianst, and the
     * code that should be run if the value matches the arm's pattern
     * 
     * The comparison takes the form
     * guess < secret_number
     * 
     * Also, the secret_number is inferred to be u32
     */ 
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too Small!"),
        Ordering::Greater => println!("Too Big!"),
        Ordering::Equal => println!("You Win!"),
    }
}