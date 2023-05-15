// Defaults suck in Rust, for functions just use Option<T>

struct A {
   val1: u32,
   val2: i32,
}

impl Default for A {
   fn default() -> Self {
        A {
            val1: 6,
            val2: -7,
        }
   }
}

pub fn run(){

    // I have to specify the type
    let obj: A = Default::default();

    let obj = A { val1: 12, ..Default::default() };

}
