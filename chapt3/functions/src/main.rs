fn main() {
    println!("Hello, functions!");
    another_banger();
    another_banger_with_params(func_with_return());
}

fn another_banger() {
    println!("Another banger!!");
}

// Multiple parameters are separated by commas.
fn another_banger_with_params(x: i32) {
    println!("Another {} bangers!!", x);
}

fn func_with_return() -> i32 {
    // implicit return: the value of the last expression in a block/ function body is returned
    // notice the absence of semi-colon. Semi-colons indicate end of statements and do not return a value.
    5 
}
