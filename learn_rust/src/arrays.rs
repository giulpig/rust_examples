// Fixed lenght, stack allocated

// For the size_of_val function
use std::mem;

pub fn run(){

    let numbers : [i32; 5] = [1,2,3,4,5];

    let numbers : [3;5]; //[3, 3, 3, 3, 3]

    // Print whole array
    println!("{:?}", numbers);

    // Get length
    let _lunghezza = numbers.len();

    // Iterate in array, iter_mut to modify
    for i in numbers.iter(){
        print!("{}", *i);
    }
    println!();

    // Get used memory
    let _mem_occupata = mem::size_of_val(&numbers);

    // Get slice
    let _slice: &[i32] = &numbers[0..2];

}
