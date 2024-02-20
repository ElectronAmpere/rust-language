// Reference and Borrowing in Rust
fn main()
{
    let s1 = String::from("Hello");
    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize
{
    s.len()
}
