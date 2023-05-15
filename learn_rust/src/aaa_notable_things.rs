pub fn run() {

    println!("-- aaa_notable_things module --");

    // 1 - return result from loop with "break" (you can see break as a return statement)

    let result_from_loop = loop {
            break 42
        };

    println!("Break from loop returned {result_from_loop}");


    // 2 - name loops to break/continue from outer loop
    'outer_loop: for _ in 0..5 {
        loop{
            if 5>3{
                println!("Breaking from outer loop");
                break 'outer_loop
            }
            continue 'outer_loop
        }
    }

}
