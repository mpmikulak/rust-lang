// The keyword mod is used to import other modules
mod rust1;

// To import use the keyword use
use std::mem;

fn main() {

    // Any function with a "!" is a macro
    println!("Hello, world!"); // Lines must end with a semi-colon

    // Double colons are how you call a function from a module
    rust1::main1();

    // Single colons override type inference
    let bool_two:bool = false;
    println!("My bool_two value is {}", bool_two);
    println!("Size of bool is {} bytes",mem::size_of_val(&bool_two)); // Use the mem::size_of_val to determine the size of a value

    let c = 'c'; // Declare a character
    println!("Size of c is {} bytes",mem::size_of_val(&c));

    let number:i32 = 42; // Different sizes available: i8,u8,i16,u16,i32,u32,i64,u64
    println!("Size of number is {} bytes",mem::size_of_val(&number));
    println!("c = {}, number = {}",c, number);

    let double_number = 1.0; // Different sized available: f32, f64
    println!("Double number = {}", double_number);
    println!("Size of double_number is {} bytes",mem::size_of_val(&double_number));

    let my_string = "Hello, String!!";
    println!("my_string = {}", my_string);
}
