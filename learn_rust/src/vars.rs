pub fn run(){

    let name = "Giulio";

    // Mutable variable - IMMUTABLE BY DEFAULT
    let mut age = 37;
    age += 1;
    println!("{} age {}", name, age);

    // Define constant - mandatory var type
    const ID: i32 = 1;
    println!("{}", ID);

    // Tuple assignment
    let (uno, due) = (1, 2);
}