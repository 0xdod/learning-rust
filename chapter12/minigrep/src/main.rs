/**
 * minigrep is a basic implementation of the popular unix command-line tool grep.
 */

use std::env; // bring this module to scope

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];
   //  println!("{:?}", args); // use debug formatter :? to print args
    println!("Searching for {}", query);
    println!("In file {}", filename);
}