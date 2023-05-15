//public function
pub fn run(){

    println!("Hello from print.rs file");

    println!("Number {}", 3);

    // Positional parameter
    println!("{0} {1} {2} {0}", "uno", "due", "tre");

    // Named parameters
    println!("{nome} {cognome}", nome="giulio", cognome="codutti");

    // Placeholder traits
    println!("Binary: {:b} hex: {:x}", 10, 10);

    // Debug trait (of a tuple)
    println!("{:?}", (12, true, "ciao"));


    // I can print all sort of things on stderr(2) too
    dbg!(2+2);
}
