
// LIFETIMES IN FUNCTIONS
// Here the lifetime "a" will be valid as long as "x" and "y" live
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len(){
        x
    }
    else{
        y
    }
}


// LIFETIMES IN STRUCTS
// "part" will be valid as long as reference used during construction of the object is valid
struct ImportantPart<'a>{
    part: &'a str
}

impl<'a> ImportantPart<'a>{    //  v-- I could have omitted this
    fn importantPhrase(&self) -> &'a str{
        self.part
    }
}


// STATIC LIFETIMES - They can live for the entire program
fn dummy(){
    let s: &'static str = "I have a static lifetime";
}



// GENERICS, LIFETIMES AND TRAITS ALL TOGHETHER
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
T: std::fmt::Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
    
    
    
    
    pub fn run(){
        
        let str1 = String::from("abcd");
        {
            let str2 = "xyz";
            let result: &str = longest(str1.as_str(), str2);
            println!("Longest is {}", result);
        }
        
        let sentences: String = String::from("Testo super importante. Testo a caso");
        let first_sentence: &str = sentences.split('.').next().expect("Not valid sentences");
        let i = ImportantPart{
            part: first_sentence
        };
        
    }
