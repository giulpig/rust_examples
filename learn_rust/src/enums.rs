// 1- Basic enum
enum BaseIpAddrType{
    V4,
    V6
}

// 2- Enum with associated values, similar to a class
enum AdvIpAddrType{
    
    // Here V4 acts like a constructor for a tuple
    V4(u8, u8, u8, u8),
    V6(String),
    Invalid(enum_of_enums)
}

enum enum_of_enums{
    BaseIpAddrType
}

// I can implement methods in enums too!
impl AdvIpAddrType{
    fn call(&self) -> bool{
        true
    }
}


// Focus on Option enum, to avoid using null
fn focus(){
    
    // The type is a Option<&str>
    let some_number = Some(5);
    
    // I have to specify the Option type if I have None (== null)
    let no_number : Option<u32> = None;

    
}



pub fn run(){

    // I can instantiate an enum just like an object
    let four = BaseIpAddrType::V4;
    let home = AdvIpAddrType::V4(127, 0, 0, 1);

    // I can use match like a superpowered switch statement
    // OKKIO: match has to cover all cases or it won't compile
    // OKKIO: match does evualuation in order so "other" or "_" must be last
    match four{
        BaseIpAddrType::V4 => {
            println!("v4");
        },
        BaseIpAddrType::V6 => {
            println!("v6");
        },
        // In any other case, equivalent to '_'
        other => {
            // Do nothing
            ();
        }
    }


    // If let statement
    let config_max : Option<u8> = Some(99);
    if let Some(max) = config_max {
        // Note: max can be used only in this scope
        println!("The max is set to {}", max);
    }
    // I can add else statement too
    else{
        println!("Not valid (None) max value");
    }



}