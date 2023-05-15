// 1- This is a struct
#[derive(Debug)]
struct User{
    active: bool,
    username: String
}

// I can implement methods like this
impl User{
    // This is a method because it takes self as first argument
    fn kill(& mut self){
        self.active = false;
    }

    // This is also a method
    fn has_same_name(&self, other: User) -> bool{
        self.username == other.username
    }
}

// I can have multiple implementations, to split methods and associated functions
impl User{
    // This is an associated function, not a method
    // To call this I have to write "User::make_user"
    fn make_user(_username: String) -> User{
        User{
            username: _username,
            active: true
        }
    }
}

// 2- This is a tuple struct
struct Color(u32, u32, u32);

// 3- This is unit-struct
struct AlwaysEqual;


pub fn run(){

    let user_obj = User{
        username: String::from("giulpig"),
        active: true
    };

    let acaso = user_obj.active;

    let user_obj2 = User{
        username: String::from("filopedo"),
        
        // This uses userObj initializer list
        // OKKIO, it uses move as default (int this case is a copy (bool))
        ..user_obj
    };

    let black = Color(0, 0, 0);

    // It works like a tuple
    //println!("{:?}", black);

    // JSON-like printing
    println!("{:#?}", user_obj);

    // Debug printing to stderr (2)
    dbg!(&user_obj);

    // I can debug all sort of things
    dbg!(acaso);
}