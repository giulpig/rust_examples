// Slices are references!!! so you have to use refs (&str)
// str type is internally a slice of memory, so it is immutable

pub fn run(){

    let s = String::from("ciao mona");

    // [0..2] === [..2]
    // [2..len()] === [2..] 
    let ciao = &s[0..s.find(' ').unwrap()];

    println!("{:?}", ciao);

    let a = [1,2,3,4];

    let pezzo : &[u32] = &a[1..3];
}