use std::thread;
use std::time::Duration;

/*
FUNCTIONS TO CLOSURES
    fn add_one_v1(x: u32) -> u32        { x + 1 }
    let add_one_v2 = |x: u32| -> u32    { x + 1 };
    let add_one_v3 = |x|                { x + 1 };
    let add_one_v4 = |x|                  x + 1 ;

    Okkio, closures are NOT GENERIC if I don't specify it
    they bind the type of the first call

    All closure implement at least one of these traits (that are infered from the compiler):
        -Fn
        -FnMut
        -FnOnce
*/

// STRUCT FOR LAZY EVALUATION => executes only when is required and stores it
struct LazyEval <T>
 where T: Fn(u32) -> u32{
    closure : T,
    result: Option<u32>,
}

impl<T> LazyEval <T>
 where T: Fn(u32) -> u32{
    pub fn new(closure: T) -> LazyEval<T> {
        LazyEval{
            closure,
            result: None,
        }
    }

    pub fn get_value(&mut self, arg: u32) -> u32{
        match self.result{
            Some(val) => val,
            None => {
                let temp = (self.closure)(arg);
                self.result = Some(temp);
                temp
            },
        }
    }
}


// Plain funciton example
pub fn intensive_function(intensity: u32) -> u32{
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn plan_workout(arg1: i32, arg2: u32){
    let intensive_closure = |num: u32 /*,param2: String*/| -> u32 {
        thread::sleep(Duration::from_secs(2));
        num
    };
    // I can call this only when I need it
    intensive_closure(arg2);
    // Now the closure is enforced to have the type of arg2 if I didn't specify it
}


//CAPTURING VARIABLES
// -FnOnce consumes the variables (move)
// -FnMut borrows with mutability
// -Fn borrows with immutability
pub fn run(){
    plan_workout(12, 7);

    let very_intersting_varable = 4;
    // Here the move keyword invalidates "very_intersting_varable" inside the closure
    let my_closure = move |num| { num+very_intersting_varable };
    assert_eq!(my_closure(7), 11);
}
