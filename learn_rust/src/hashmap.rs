use std::collections::HashMap;


pub fn run(){

// Creation
    let mut scores: HashMap<String, i32> = HashMap::new();

    let nums_str: Vec<String> = vec!["Dieci".to_owned(), "Nove".to_owned()];
    let nums_int: Vec<i32> = vec![10, 9];

    // Initalization through iteration, Rust can infer type here
    let mut scores: HashMap<_, _> =
        nums_str.into_iter().zip(nums_int.into_iter()).collect();


// Insertion
    // OKKIO: .insert() takes ownership (moves) inserted objects
    scores.insert(String::from("Dieci"), 10);
    scores.insert(String::from("Nove"), 9);

// Getting an element
    let number = scores.get("Dieci");  //OKKIO: .get() returns an Option<&T>
    match number{
        Some(number) => println!("{}", number),
        None => println!("Dieci not in the map")
    }

// Looping 
    for (key, val) in &scores{
        println!("{}: {}", key, val);
    }

    

}
