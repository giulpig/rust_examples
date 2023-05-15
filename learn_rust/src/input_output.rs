use std::io;
use std::io::Write;

pub fn run(){
    let mut guess = String::new();

    print!("Insert a string: ");

    // Flush output
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    // There is a newline
    guess.pop();

    println!("{} has been read", guess);
}