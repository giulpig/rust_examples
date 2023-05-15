// Primitive str: immutable
// String: heap allocated data struct

// OKKIO: I can NOT do "mia_stringa[0]"

pub fn run(){

// Only a constant, not very useful
    let hello: &str = "helloWoow";

    let mut hello_str: String = String::from("hello"); //= "hello".to_string();

// String to a reference to array of bytes
    let bytes: &[u8] = hello.as_bytes();

// Push only for characters
    hello_str.push('W');

// Push_str for strings
    hello_str.push_str("oow");

// Concatenate two strings
    let s1: String = "str1".to_owned();
    let s2: String = "str2".to_owned();
    let s3: String = "str3".to_owned();
    // OKKIO: This moves s1 ownership to concat so s1 is not accessible
    let concat: String = s1 + &s2 + &s3;

// Format strings - less efficient than concatenation but easier to read
    let formatted = format!("{}-{}", s2, s3);

// Get lenght
    println!("{}", hello.len());

// Contains
    let contains: bool = hello.contains("Woow");    //compare is true

// Replace, does not modify source
    println!("Replace: {}", hello.replace("Woow", "replaced"));

// Assertion test, hello==hello_str is true
    assert_eq!(hello, hello_str);

// Loop through string
    for carattere in hello.chars() {
        print!("{} ", carattere);
    }
    println!();
    
    for byte in hello.bytes() {
        print!("{} ", byte);
    }
    println!();

// String slices
    let hello: String = "дравствуйте".to_string();  //hello can be a slice (&str) too
    let slice: &str = &hello[0..4];  //note, if I write [0..3] I have a runtime error

    // Actually prints "др" - not "драв"
    println!("{}", slice);
    
    

}

// If I take string slices I can take String too (compiler can coerce the reference)
pub fn first_word(s: &str) -> usize{
    
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }

    // This is a return
    s.len()

}