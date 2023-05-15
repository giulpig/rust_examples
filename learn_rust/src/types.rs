/*
 INT: u8, u16, u32, u64, u128, i8...
 FOAT: f32, f64 [3.45]
 BOOL: bool [true]
 CHAR: char ['a']
 Tuples
 Array
*/

pub fn run(){

    let x: u128 = 32;

    // Find max size
    println!("Max u128: {}", std::u128::MAX);

    let my_tuple = (1,2,3);
    println!("{:?}", myTuple);
}
