use std::collections::HashMap;
use std::io;

fn main() {
  ex1();
  println!("apple: {}\nfirst: {}", ex2("apple"), ex2("first"));
  ex3();
}

fn ex1() {
    /*
    Given a list of integers, use a vector and return the mean (the average
    value), median (when sorted, the value in the middle position), and mode
    (the value that occurs most often; a hash map will be helpful here) of the
    list.
    */
    let mut ints = vec![1,2,3,4,5,6,7,8,9, 2, 10, 10, 2,2];
    let mut sum = 0;
    let n = ints.len();
    
    for int in &ints {
        sum += *int;
    }

    println!("mean value of list {:?} is {}", ints, sum / n);

    // sort vector if i32
    ints.sort();
    let mut _n = n;
    if n % 2 != 0 {
        _n += 1;
    }

    println!("median position of list {:?} is {} which contains the value {}", ints, n/2, &ints[(_n/2) -1]);

    let mut map = HashMap::new();

    for v in &ints  {
        let count = map.entry(*v).or_insert(0);
        *count += 1;
    }
    
    let mut highest = 0;
    let mut mode = 0;

    for (k, v) in &map {
        if *v > highest {
            highest = *v;
            mode = *k;
        }
    }

    println!("The mode of list {:?} is {} with count {}",ints, mode, highest);
}


fn ex2(s: &str) -> String {
     /*
    Convert strings to pig latin. The first consonant of each word is moved to
    the end of the word and “ay” is added, so “first” becomes “irst-fay.”
    Words that start with a vowel have “hay” added to the end instead
    (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8
    encoding!
    */

    let consonants = String::from("bcdfghjklmnpqrstvwxyz");
    let ay = "ay";
    let hay = "hay";

    let first_char = s.chars().next().unwrap();
    let mut is_consonant = false;

    for c in consonants.chars() {
        if first_char.to_lowercase().next().unwrap() == c {
            is_consonant = true;
            break;
        }
    }

    let mut ss = String::new();
    let mut count = 0;

    for c in s.chars() {
        if is_consonant {
            if count != 0 {
                ss.push(c);
            }
        }else {
            ss.push(c);
        }
        count+=1;
    }

    ss.push('-');
    
    if is_consonant {
        ss.push(first_char);
        ss.push_str(ay);
    }else {
       
        ss.push_str(hay)
    }

    ss
}

fn ex3() {
    /*
Using a hash map and vectors, create a text interface to allow a user to add
employee names to a department in a company. For example, “Add Sally
to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of
all people in a department or all people in the company by department,
sorted alphabetically.
    */
    let options = vec![
        "add user to a department",
        "list users in a department",
        "get all employees by department",
        "quit",
    ];

    println!("-------------Welcome to my company.--------------");
    // HashMap<String, Vector<String>>
    let mut company = HashMap::new();

    loop {
        println!("Select an option to proceed.");
        let mut i = 1;
        for opt in &options {
            println!("{}: {}", i, opt);
            i += 1;
        }

        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Failed to read line");

        let option: i32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => -1,
        };
         
        match option {
            -1 => println!("An error occured!"),
            1 => add_user_to_dept(&mut company),
            2 => list_users_in_dept(&company),
            3 => all_users_by_dept(&mut company),
            4 => break,
            _ => println!("Invalid selection!"),
        };

    }
}

fn add_user_to_dept(company: &mut HashMap<String, Vec<String>>) {
    let mut user = String::new();
    let mut dept = String::new();
    println!("Enter department name:");
    io::stdin().read_line(&mut dept).expect("failed to read line");
    println!("Enter employee name:");
    io::stdin().read_line(&mut user).expect("failed to read line");
    let dept = company.entry(dept.trim().to_string()).or_insert(Vec::new());
    dept.push(user.trim().to_string());
}

fn list_users_in_dept(company: &HashMap<String, Vec<String>>) {
    let mut dept = String::new();
    println!("Enter department name:");
    io::stdin().read_line(&mut dept).expect("failed to read line");

    let employees = match company.get(&dept.trim().to_string()) {
        Some(emp) => emp,
        None => {
            println!("No employee in this department");
            return
        },
    };

    for emp in employees {
        println!("{}", emp);
    }
}

fn all_users_by_dept(company: &mut HashMap<String, Vec<String>>) {
    for (dept, users) in company {
        println!("{} department", dept);
        users.sort();
        for user in users {
            println!("- {}", user);
        }
    }
}