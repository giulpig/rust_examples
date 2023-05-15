// Recovable and Unrecovable errors
// "RUST_BACKTRACE=1 cargo run" to have backtrace of error (debug symbol is required)
// "RUST_BACKTRACE=full cargo run" for full backtrace


pub fn run(){

    // Printing to stderr
   eprintln!("This goes to stderr");

// UNRECOVABLE ERRORS - panic!()

    //panic!("Crashing");

    // Example of crashing
    //let v = vec![1,2,3];
    //println!("{}", v[99]);


// RECOVABLE ERRORS - Result<T, E>

    // Result enum can be Ok(T) or Err(E)
    use std::fs::{File, self};
    use std::io::{ErrorKind, Read};

    let f: Result<File, std::io::Error> = File::open("errors.rs");

    let f = match f{
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("errors.rs") {
                Ok(fc) => fc,
                Err(e) => panic!("Unable to create the file: {:?}", e),
            },
            other_error => {
                panic!("Other error detected: {:?}", other_error);
            }
        },
    };

    // unwrap() method, returns file if exists, else calls panic!()
    let f: File = File::open("errors.rs").unwrap();

    // expect() is like unwap() but you can specify panic message
    let f: File = File::open("errors.rs").expect("Cannot open file \"errors.rs\"");


// PROPAGATION OF ERRORS, done by returning a Result enum, see better way in next function
    fn propagate_error() -> Result<String, std::io::Error>{

        let f = File::open("cacca.tzt");

        let mut f = match f {
            Ok(file) => file,
            Err(error) => return Err(error),
        };

        let mut str = String::new();

        match f.read_to_string(&mut str) {
            Ok(_size) => Ok(str),
            Err(error) => Err(error),
        }

    }

// THE "?" OPERATOR

    // Nice (almost) equivalent shortcut, (actually calls the "from" function to convert errors)
    fn propagate_error_shortcut() -> Result<String, std::io::Error>{

        let mut str = String::new();

        let mut f = File::open("cacca.tzt")?;   //Here is like "return Err(e) if fails, else go ahead"
        f.read_to_string(&mut str)?;

        // Or, in just one line
        let mut f = File::open("cacca.tzt")?.read_to_string(&mut str)?;

        Ok(str)
    }

    // However the shortest way is this:
    fn propagate_error_supershortcut() -> Result<String, std::io::Error>{
        fs::read_to_string("cacca.tzt")
    }

    // I can use the "?" operator with Option<T> too
    fn first_char(text: &str) -> Option<char>{
        text.lines().next()?.chars().next()
    }
}


// MAIN FILE CAN RETURN A RESULT (with a dynamic error in a Box)

use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error> > {

    let f = File::open("cacca.tzt")?;
    Ok(())

}


// STRUCT FOR VALIDATING USER'S INPUT ERROR

pub struct Guess{
    value: i32
}

impl Guess{
    pub fn new(value: i32) -> Guess{

        if value<0 {
            panic!("No negative values, got {}", value);
        }

        Guess{value}
    }

    pub fn value(&self) -> i32{
        self.value
    }
}
