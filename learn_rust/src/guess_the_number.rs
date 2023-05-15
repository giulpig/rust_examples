use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub fn run() {

    println!("Guess the number");
    // "..= range" ==> inclusive
    let number = rand::thread_rng().gen_range(1..=2);

    //println!("{}", number);

    // I can assign loop returns to variable
    let risultato : &str = 
    {
        // I can have loop labels
        'loop_label: loop{
    
            let mut guess = String::new();
            
            io::stdin()
                .read_line(&mut guess)
                .expect("Error in IO operation");

            // Converto guess to a u32 number
            let guess : u32 = match guess.trim().parse(){
                Ok(num) => num,
                Err(_) => continue 'loop_label,
            };

            // match is like a switch
            match guess.cmp(&number){
                Ordering::Less => println!("Too small"),
                Ordering::Greater => println!("Too big"),
                Ordering::Equal => {
                    // This is like a return
                    break "Benone";
                }
            };
        }
    };

    // Should be "Benone"
    println!("{}", risultato);
}
