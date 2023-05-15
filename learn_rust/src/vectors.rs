// Variable length, heap allocated

use std::mem;


pub fn run(){

    let mut numbers : Vec<i32> = vec![1,2,3,4,5];
    
    // Print whole vector
    println!("{:?}", numbers);

    // Get length
    let _lunghezza = numbers.len();
    
    // Iterate in vector, iter_mut to modify
    for i in numbers.iter_mut(){
        print!("{}", *i);
        *i+=1;
    }
    println!();

    // Get used memory
    let _mem_occupata = mem::size_of_val(&numbers);

    // Get slice
    let _slice: &[i32] = &numbers[0..2];

    // Add element
    numbers.push(6);

    // Pop value
    numbers.pop();
    
}