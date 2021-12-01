// Declaring constants: Type annotations are necessary, and the value must be known at compile-time
// Code:
//const MAX_POINTS :u32 = 100_000;

fn main() {
    println!("Hello, variables!");
    // Variable mutability
    let mut x = 5;
    println!("x = {}", x);
    x = 6;
    println!("x = {}", x);
    // Variable shadowing
    // The two variables could even be of different types.
    let y = 7;
    let y = y * 10;
    println!("y = {}", y);

    // Compound types include Tuples and Arrays
    // Tuples can hold elements of different types and have a fixed length.
    // type annotation is not necessary
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // Tuples can be destructured as follows:
    let (x, y, z) = tup;
    println!("x = {}, y = {}, z = {}", x, y, z); // notice subtle use of shadowing on x and y in this scope?
    // The dot notation can also be used with (zero-based) index to access tuple elements:
    let x = tup.0;
    println!("again, x = {}", x);

    // Array type:
    // Arrays are fixed-length collection structures holding values of same type
    let _arr: [i32; 5] = [1,2,3,4,5];
    // Accessing array elements is similar with other PLs e.g let el = arr[0]
    let _arrx = [3; 10]; // This will create an array that has element 3 repeated 10 times.
    // Arrays going out of bounds cause a panic
}
