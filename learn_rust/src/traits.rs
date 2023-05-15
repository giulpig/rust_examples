// Similar to interfaces


pub trait Summary {
    // Default summarize method for all, I can call other (not default) methods too
    fn summarize(&self) -> String{
        format!("{}, read more...", self.get_title())
    }
    fn get_title(&self) -> String;
}
pub trait Void{}


pub struct Article{
    title: String,
    paragraphs: Vec<String>
}
// I use the "for" keyword to implement a trait
// Note: I have to implement ALL trait's methods that have no default implementation
impl Summary for Article{
    fn summarize(&self) -> String{
        self.paragraphs[0].clone()  //I copy the string
    }
    fn get_title(&self) -> String{
        self.title.clone()
    }
}


pub struct Tweet{
    title: String,
    text: String
}
impl Summary for Tweet{
    fn summarize(&self) -> String{
        self.text.clone()
    }
    fn get_title(&self) -> String{
        self.title.clone()
    }
}


// PASSING TRAITS TO FUNCTIONS        v-- Note: I still have to know the return type at compile time even if I can specify a trait
fn notify(item: &(impl Summary+Void)) -> impl Summary{
    println!("{}", item.summarize());
    Tweet{
        text: String::from(""),
        title: String::from("")
    }
}
// This is actually the same thing but more verbose
fn notify2<T: Summary+Void>(item: &T){
    println!("{}", item.summarize());
}
fn notify3<T, U>(item1: &T, item2: &U)
where T: Summary+Void,
U: Summary
{
    println!("{}", item1.summarize());
}


// FUNCTIONS THAT IMPLEMENT COPY AND ORDER TRAITS
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}


// BLANKET IMPLEMENTATIONS  - IMPLEMENTING SUMMMARY TRAIT ON TOP OF VOID TRAIT
impl<T: Void> Summary for T {
    // --snip--
    fn get_title(&self) -> String{
        String::from("Void - no title")
    }
}


pub fn run(){
    
    let my_tweet = Tweet{title: String::from("mozzarella"), text:String::from("molto buona")};
    let my_article = Article{title: String::from("pizza"), paragraphs: vec!(String::from("la pizza"), String::from("e' molto buona"))};
    
    println!("{}", my_tweet.summarize());
    println!("{}", my_article.summarize());
    
    
}