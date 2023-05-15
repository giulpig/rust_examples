pub fn run(){
    greetings("mandi", "ciccio");
    let _somma = add(5,5);

    // Closure, similar to lambda
    let add_nums = |n1:i32, n2:i32| n1+n2;
    println!("{}", add_nums(2,3));

    let miaStringa = String::from("ciao");
    take_ownership(miaStringa);
    // Now miaStringa is invalid!!!! (because every "=" is a move)
}

fn greetings(greet: &str, name: &str){
    println!("{} {}", greet, name);
}


fn take_ownership(greet: String){
    println!("{}", greet);
}


fn add(n1:i32, n2:i32) -> i32{
    
    n1+n2   //same as return n1+n2;
}

fn never_returning() -> !{
    panic!("PanicoPaPanicoPauraa - Stopping thread and not returning");    
}