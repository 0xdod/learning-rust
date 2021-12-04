
// Derived trait Debug useful for printing debug information.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle{ width: 30,height: 50 };
    let square = Rectangle::square(25);
    let circle = Circle(5.0);
  // let Circle(x) = circle; // tuple struct destructuring
    
    println!("circle area is {}",circle.area());
    
    println!("The area of the rectangle is {} square pixels.", rect.area());
    println!("rect>>> {:#?}", rect);

    println!("The area of the square is {} square pixels.", square.area());
    println!("sq>>> {:#?}", square);
}

// fn area(rect :&Rectangle) -> u32 {
//     rect.width * rect.height
// }

// Method Syntax
// To define a method, we create an implementation block and the function
// The first parameter in a method is always self (no need for type annotation for self).
// Methods can take ownership of the instance (rare), can borrow mutably and immutably the instance
// &mut self for mutable borrow if the method changes the instance properties
// Associated fns don't take self as a parameter, but are defined in the impl block of a type
// Associated fn are useful for constructors :: is used to call the fns 
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // square is an associated function
    fn square(size: u32) -> Rectangle {
        Rectangle{width: size, height: size}
    }
}

struct Circle(f64);

impl Circle {
    fn area(&self) -> f64 {
        self.0 * self.0 * 3.14
    }
}
