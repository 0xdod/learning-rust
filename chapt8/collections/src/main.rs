fn main() {
    //  -Vector: A collection of similar types, really flexible over arrays and tuples.
    // like arrays can hold only similar types, unlike arrays does not have a fixed length
   // let v: i32 = Vec::new(); -> type annotation needed for this, Alternatively, you can do:
   let mut v = vec![1,2,3,4,]; // Type is automatically inferred by rustc
   v.push(5); // Update vector. A vec is dropped when it goes out of scope
   let _third: &i32 = &v[2]; // read from a vec
   
   match v.get(2) { // read from a vec
       Some(third) => println!("{}", third),
       None => println!("No value"),
   };

   // if we don't use references, the vector would be owned in the new for scope the for loop created,
   // and cannot be accessed after the for loop scope because it was moved. Instead we borrow from the vector. 
   for i in /*&mut v for mutable references */ &v {
       println!("{}", i);
   }

   // We can use enums to store multiple types in a vector. Variants of an enum are still an enum.
   // We can create an enum with varaints containing different types and put them in a vector
   enum SpreadsheetCell {
    Int(i32),
    Text(String),
    Float(f64),
   }

   let _rows = vec![
    SpreadsheetCell::Int(1),
    SpreadsheetCell::Text(String::from("hello world")),
    SpreadsheetCell::Float(42.5),
   ];

   // Strings
   let _s = String::new(); 
   let data = "initial data";
   let mut data_string = data.to_string(); // same as String::from()
   // We can grow a string by:
   data_string.push_str("new data"); // add string slice to String because we don't want to take ownership of the parameter
   data_string.push('w'); // adds a single char to String
   // To combine two existing Strings, one option is to use the + operator
   let s1 = String::from("hello");
   let s2 = String::from("world");
   let s3 = s1 + &s2; // add(self, s &str) -> String; this is the format for string concatenation
   // s1 is invalid cause it has moved, s2 is still valid and was borrowed
   // For complex concatenation, user format!
   let s4 = format!("{} {}", s2, s3);
   // you can't index strings in rust, but you can get a slice of strings. (should be used with caution)
   // to get individual string chars, use the .chars() or bytes() methods
   for v in s4.chars() {
    println!("{:?}", v);
    }

    // HashMaps -: they are of the type HashMap<K, V> where K is key of type K and V is value of type V
    use std::collections::HashMap;
    let mut score = HashMap::new();
    score.insert(String::from("Blue"), 30);
    // Alternative way to create a HashMap from vector of tuples withc collect()
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect(); 
    match scores.get(&String::from("Blue")) {
        Some(val) => println!("{}", val),
        None => println!("None"),
    }

    // to loop over a map
    for (k, v) in &scores {
        println!("{}: {}",k, v);
    }
    // checks if entry exists in map if yes return a mut ref to the value,
    // if no insert the entry and return a mut ref of the new value
    scores.entry(String::from("White")).or_insert(100);


}


