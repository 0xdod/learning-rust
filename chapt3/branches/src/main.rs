fn main() {
    println!("Hello, branches!");

    let conditon = true;
    // Rusts supports traditional if--else-if--else blocks
    if conditon {
        println!("Condition is true");
    }else {
        println!("Condition is false");
    }

    // if-expression can also return a value
    let a = if conditon {
        6 // notice no semicolon to return this expression from this block
    }else {
        7 // no semi-colon
    }; // notice semi-colon to end the statement, and both if and else block must return same type.
    println!("a = {}", a);
}
