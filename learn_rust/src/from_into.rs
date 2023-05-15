use std::convert::From;

#[derive(Debug)]
struct NumberAndBase {
    value: i32,
    base: i32,
}

// Implement the From trait
impl From<i32> for NumberAndBase {
    fn from(item: i32) -> Self {
        NumberAndBase {value: item, base: 10}
    }
}

pub fn run(){

    // I can convert it using the From method
    let num = NumberAndBase::from(30);
    println!("Number {:?}", num);

    // I can call Into "for free" (NOTE, I can't do the reverse conversion for free though)
    let int = 5;
    let num: NumberAndBase = int.into();
    println!("Number {:?}", num);
}
