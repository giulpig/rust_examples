
// GENERIC FUNCTION
fn first<T> (arg1: T, arg2: T) -> T {
    arg1
}

// GENERIC STRUCT
struct Point<T, U>{
    x: T,
    y: U
}
impl<T, U> Point<T, U> {
    fn new (x: T, y: U) -> Point<T, U> {
        // I can omit the "x:x" and "y:y" because they have the same name
        Point{x, y}
    }
    // I can have multiple template inside templates
    // Note: this function invalidates both self and other objects
    fn mixup<Tother, Uother> (self, other: Point<Tother,Uother>) -> Point<T, Uother>{
        Point{x: self.x, y: other.y}
    }
}
impl Point<f32, f32>{
    // This function is implemented only on f32,f32 points
    fn rounding (&self) -> Point<i32,i32>{
        Point{x:self.x.round() as i32, y:self.y.round() as i32}
    }
}


// GENERIC ENUM
enum My_Option<T>{
    Something(T),
    Nothing
}


pub fn run(){

    assert_eq!(4, first(4, 5));
    assert_eq!('a', first('a', 'n'));

    // Note: constructor is equal to Point{x:45, y:45};
    let punto: Point<u32, f64> = Point::new(4, 5.7);

}