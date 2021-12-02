// Ownership is the mechanism used in Rust to manage heap data.
// Every value has a variable called owner
// There can be only one owner at a time
// When the owner goes out of scope, the value will be dropped
// Important terms:
//      Move:
//      Clone:
//      drop and Drop trait
//      copy and Copy trait
// Ownership rules for assignment function calls are same.
// Return calls can also transfer ownership 
// We can use references and borrowing to avoid the ceremony of taking ownership and giving it back all the time when we pass values to a function.
// borrowing is when a fuction takes a reference parameter
// references are immutable by default
// to make a reference mutable, use the mut keyword in the fuction call and decalration
// e.g fake_fn(s: &mut String)     // fake_fn(&mut s)
// There cannot be more than one mutable reference of a particular variable in a given scope
// Mutable and Immutable references in a given scope is unacceptable 

fn main() {
    println!("Hello, Ownership");

    let  s = String::from("hello from heap"); // request heap memory

    let s1 = s;
    // s is invalid at this point, ownership of the value has been moved to s1.
   
    // instead of moving ownership, we can pass a reference parameter to a fn.
    let _len = calculate_len(&s1); // borrows s1
    
    let word = first_word(&s1);

    println!("First word is {}", word);

    takes_ownership(s1);

    // s1 is now invalid, as it has been moved
    // println!("{} is now invalid and this should be a compile error", s1);

    let x = 42;

    makes_copy(x);
    // x is still valid, a copy was passed

    // after this fn, x will be out of scope and popped from the stack
    // s will be out of scope, since it's already invalid nothing happens 

    
}

fn takes_ownership(s: String) {
   println!("value '{}' is moved and only valid here", s);
    // owner s is out of scope and dropped
}

fn makes_copy(i: i32) {
    println!("value '{}' is copied", i);
    // i goes out of scope and popped off stack
}

fn calculate_len(s: &String) -> usize {
    return s.len();
}

// A string slice is a reference to part of a String.
// &str is the data type. The range syntax [i..j] is used to slice a string from i to j-1
 
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}