
// Primitive datatypes
// int, float, bool, char

//Integers
//Rust has signed (+, -) and unsigned integers
// (only+) types of different sizes.

// i8, i16, 132, i64, i128: are signed
// u8, u16, u32, u64, u128: are unsigned

fn main(){
    let x: i32 = -42;
    let y: u32 = 100;

    println!("The signed integer: {}", x);
    println!("The unsigned integers: {}", y);


//Floats
// f32, f64
    let pi: f64 = 3.14;
    println!("The float number {}", pi);

//Boolean

let is_compleing: bool = true;
println!("The boolean value: {}", is_compleing);

// Character -char
let letter: char = "a";
println!("The first letter of alphabet: {}", letter)
}
