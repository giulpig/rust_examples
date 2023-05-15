pub fn run(){

    let mut count = 0;

    // Infinite loop
    loop{
        break;
    }

    while count < 20{
        println!("inside while");
        count += 20;
    }

    // Note: 100 is excluded
    for i in (0..100).rev(){
        count += 1;
    }
}