// There are some superpowers that come with usafe rust
//   - Dereference a raw pointer
//   - Call an unsafe function
//   - Access or modify a mutable static variable
//   - Implement an unsafe trait
//   - Access fields of unions


unsafe fn very_unsafe(){
}

pub fn run(){
    let mut num = 5;

    // I can cast &num to raw pointers in safe rust, because the compiler knows these refs are valid
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // Direct pointer to memory
    let r3 = 0x12345usize as *const i32;

    // Derferecing raw pointers is unsafe
    unsafe{
        println!("{}", *r1);
        println!("{}", *r3); //This will likely throw a seg fault

        very_unsafe(); //I have to call an unsafe function in an unsafe block
    }



    // I can call an extern (unsafe) C function like this
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe{
        println!("{}", abs(-3));
    }

    // I can call Rust function from C too
    #[no_mangle]
    pub extern "C" fn call_from_c(){
        println!("Just called a Rust function from C");
    }



    // Modify static variables
    static mut COUNTER: i32 = 0;
    unsafe{
        COUNTER += 1;
    }




}
