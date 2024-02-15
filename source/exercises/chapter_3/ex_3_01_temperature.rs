// Rust Code for Converting Temeperature 
fn main()
{
    let mut fahr: i32;
    let mut celcius: i32;
    let lower: i32 = 0;
    let upper: i32 = 300;
    let step: i32 = 20;
    
    fahr = lower;

    while fahr <= upper {
	celcius = 5 * (fahr - 32) / 9;
	println!("f:{}, c:{}\n",fahr, celcius);
	fahr = fahr + step;
    }
}
