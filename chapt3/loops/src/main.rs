fn main() {
    println!("Hello, loops!");
    // There are 3 looping constructs in rust: loop, while and for

    //----loop keyword:----
    // NOTE: Infinite loop
    // loop {
    //     println!("I am looping")
    // }

    // loop can return a value too
    let mut counter = 0;

    let val = loop {
        counter += 1;

        if counter == 5 {
            break counter; // The value after break is returned. Notice semicolon?
        }
    }; // notice semicolon to end statement

    println!("Value is {}", val);

    //-----While loops-----
    // Runs as long as condition evaluates to true


    while counter != 0 {
        println!("counter {}", counter);
        counter -= 1; // remember this condition to avoid infinite loop
    }

    println!("Gbese");

    //---For loops are mostly used to loop through a collection---
    let coll = [1,2,3,4,5];

    for el in coll.iter() {
        println!("value is: {}", el);
    }

    // For loops can be used with a range 
    for num in 1..5 {
        println!("{}!", num);
    }
}
