// Enums are a powerful concepts in Rust
// Enums allow you to define a type by enumerating all possible values it could have
// Enums can have variants/values that contains data
// All variants of an enum are of the same type.
// Enum variants are created by using the :: operator on the enum name e.g Option::Some
// The Option<T> enum is a powerful enum in Rust
// Option<T> enum is used to desribe whether a value is null or not. Rust doesn't have nulls.
// Option<T> has 2 variants Some(T) or None.
// 

enum Gender {
    Male,
    Female,
    Other(String),  // encodes data
}

fn main() {
// using Option<T> which is present globally ( brought into scope in the prelude)
//
//   let _some_value = Some("value"); // T == &str
//    let _another_value = Option::Some("value"); // You dont necessarily have to use Option:: for Option enum.
//    let _null_value: Option<String> = None; // declaring a null value in rust

    let confusion = Gender::Other(String::from("genderfluid"));
    // if let Gender::Male = confusion {
    //     println!("man")
    // }else {
    //     println!("confused")
    // }

    print_gender(confusion);
}

fn print_gender(gender: Gender) {
    // matches are exhaustive, i.e yo must check all possiblites
    // the exhaustiveness can be bypassed by using placholder value `_` as the last pattern. You won't have to check all possiblites
    // `if let` idiom is an improvement over the placholder technique. It is useful if we only want to match one thing.
    // if let Gender::Male = gender { do this}  
    match gender {
        Gender::Male => println!("Na man be this"),
        Gender::Female => println!("Na woman be this"),
        Gender::Other(custom) => println!("Wahala be like bicycle. This gender na {} ", custom),
    };
}
