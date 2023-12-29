//! Discussing Variables and Mutability
//! -----------------------------------
//! 

// ---------------- const -------------------------//
// `const` are immutable, cannot change mutability, are set only to constant expression
// not the result value of the value that can be computed only at runtime
// These are computed at compile time
// Note: seems similar to #define in C
// Ref: https://doc.rust-lang.org/reference/const_eval.html
const HOURS_IN_SECONDS: u32 = 60 * 60 * 1;
// ---------------- const -------------------------//

/// Main function
fn main() {

    // ---------------- mutability -------------------------//
    // By default the let x = 0; is a immutable variable
    // Mutabiliy of the variable x can be modified by specifying `mut` keyword
    let mut x = 0;
    println!("The value of x {x}");
    x = 6;
    println!("The value of x {x}");
    // ---------------- mutability -------------------------//

    // ---------------- const -------------------------//
    println!("Seconds in an Hour: {HOURS_IN_SECONDS}s");
    // ---------------- const -------------------------//

    // ---------------- shadowing -------------------------//
    let y = 5; // initialy y is 5
    let y = y + 1; // shadowed, now y is (5 + 1)

    {
        let y = y * 5; // shadowed again, locally modified now y is (6 * 5)
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y is: {y}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("Spaces: {spaces}");
    // ---------------- shadowing -------------------------//

    // ---------------- datatypes -------------------------//
    // The value to be parsed can be specified directly too
    // using "42".parse() instead of guess.trim() from guessing_game
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Guess: {guess}");
    // Datatype Subsets: scalar and compound
    // Scalar Type: 
    // ---------------- datatypes -------------------------//
}