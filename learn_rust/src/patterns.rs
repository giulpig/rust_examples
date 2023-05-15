pub fn run(){
    // I can use "if let" statements, "else if let" and so on with Option
    let fav_color : Option<&str> = None;

    if let Some(color) = fav_color{
        println!("{} is my favourite color", color);
    }



    // I can also use "while let"
    let mut stack = vec![1, 2, 3];

    while let Some(top) = stack.pop(){
        println!("{}", top);
    }


    // Also "let a = 5" is a pattern, the general is "let PATTERNS = EXPRESSION"

    // Also function parameters are patterns (e.g.: "&(x,y): &(i32,i32)" )


    // Patterns can be refutable or irrefutable
    //  Patterns that can fail are refutable (e.g.: let Some(x)=some_option)
    //  Patterns that cannot fail are irrefutable (e.g.: let x=5)

    match fav_color{
        Some("green") | Some("blue") => println!("green or blue"),
        _ => println!("Some other color"),
    }

    let some_number = Some(5);

    // Range-based match
    match some_number {
        Some(1..=5) => println!("small number"),
        _ => println!("other number"),
    }


    // I can deconstruct and reconstruct in the same way mirroring on the "=" sign
    struct Point{
        x: i32,
        y: i32,
    }
    let ((feet, inches), Point{x,y}) = ((3,10), Point{x:3, y:-10});


    // @ operator, it matches and assigns match pattern to a variable in the scope of the match
    match some_number {
        Some(range @ 1..=5) => println!("small number, it stays in range {}", range),
        _ => println!("other number"),
    }


    // Match and references
    let reference = &4;
    match reference {
        // If `reference` is pattern matched against `&val`, it results
        // in a comparison like:
        // `&i32`
        // `&val`
        // ^ We see that if the matching `&`s are dropped, then the `i32`
        // should be assigned to `val`.
        &val => println!("Got a value via destructuring: {:?}", val),
    }


    // Accordingly, by defining 2 values without references, references
    // can be retrieved via `ref` and `ref mut`.
    let value = 5;
    let mut mut_value = 6;

    // Use `ref` keyword to create a reference.
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // Use `ref mut` similarly.
    match mut_value {
        ref mut m => {
            // Got a reference. Gotta dereference it before we can
            // add anything to it.
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }

    // Guards
    let number: u8 = 4;
    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        // I still have to include this, because the compiler does not check guards
        _ => unreachable!("Should never happen."),
    }


    // Binding (use @)
    fn some_number(x: u32) -> Option<u32> {
        Some(42)
    }
    match some_number(88) {
        // Got `Some` variant, match if its value, bound to `n`,
        // is equal to 42.
        Some(n @ 42) => println!("The Answer: {}!", n),
        // Match any other number.
        Some(n)      => println!("Not interesting... {}", n),
        // Match anything else (`None` variant).
        _            => (),
    }

}
