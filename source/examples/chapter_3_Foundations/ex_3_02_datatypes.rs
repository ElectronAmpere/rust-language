//! Datatypes in Rust
//! -----------------

// main function
fn main() {

    // ****************** SCALAR TYPES **************** //
    // ---------------- float -------------------------//
    let x = 2.0; //f64

    let y: f32 = 3.0; //f32

    println!("x = {x} and y = {y}");
    // ---------------- float -------------------------//

    // ---------------- boolean -------------------------//
    let t = true; // implicit boolean type

    let f: bool = false; // explicit type annotation

    println!("t = {t} and f = {f}");
    // ---------------- boolean -------------------------//
    
    // ---------------- characters -------------------------//
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("c = {c}, z = {z} and heart_eyed_cat = {heart_eyed_cat}");
    // ---------------- characters -------------------------//

    
    // ****************** COMPOUND TYPES **************** //
    // ---------------- tuple -------------------------//
    let tuple: (i32, f64, u8) = (500, 6.4, 2); //(type1, type2, type3,) = tuples
    let (_x, y, _z) = tuple; // destructuring, unused variables should start with `_`
    //println!("tuple = {tuple}"); // generates error when printing it
    println!("The value of y: {y}");
    println!("The value of tuple: tuple.0 = {}, tuple.1 = {}, tuple.2 = {}",tuple.0, tuple.1, tuple.2);

    let _i: i32 = 0;
    //println!("The value of i: {i} and tuple.i {}", tuple.i); // generates error
    // ---------------- tuple -------------------------//
    
    // ---------------- array -------------------------//
    let _array = [1, 2, 3, 4, 5];
    //println!("The value of array: {array}"); // generates an error

    let array: [i32; 5] = [1, 2, 3, 4, 5]; // Form 1: array: [array type; array length] = [value1, value2, ..., valueN];
    println!("The value of array[{}] = {}", 0, array[0]);
    let array = [3; 5]; // Form 2: array = [init value; length of array];
    println!("The value for array[{}] = {}", 0, array[0]);
    // ---------------- array -------------------------//
}
