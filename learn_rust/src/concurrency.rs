use std::thread;
use std::time::Duration;
use std::sync::mpsc; //MultipleProducerSingleConsumer
use std::sync::Mutex;
use std::sync::Arc;

pub fn run(){

//BASIC THREAD SPAWNING AND JOINING

    let my_var: u32 = 4;

    // Here I move the variable into the thread
    let thread_handle: thread::JoinHandle<()> = thread::spawn(
        move || {
            println!("Spawned a new thread!, my_var is {}, Sleeping for 2 millis", my_var);
            thread::sleep(Duration::from_millis(2));
            println!("Selpt well");
        }
    );

    println!("Hello, this is the main function");
    thread::sleep(Duration::from_millis(9));
    println!("Finished main function");

    // I wait till the thread finishes
    thread_handle.join().expect("Error joining");



//MESSAGE SYSTEM

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move | | {
        let vals = vec![
            String::from("mandi"),
            String::from("form"),
            String::from("the"),
            String::from("spawned"),
            String::from("thread"),
        ];

        // Okkio: here I move the val variable into the messaging system
        for val in vals{
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(2));
        }


    let tx2 = tx.clone();
    thread::spawn(move | | {
        let vals = vec![
            String::from("mandi2"),
        ];

        // Okkio: here I move the val variable into the messaging system
        for val in vals{
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_millis(2));
        }
    });

    // I don't know why but I have to drop tx
    drop(tx);

    // rx.recv() waits for the message
    for recieved in &rx{
        println!("I recieved this message: \"{}\"", recieved);
    }

    // I can also do polling like this with try_recv()
    let recieved = rx.try_recv().unwrap_or_else(|_|{String::from("Nothing")});



// SHARED OWNERSHIP

    // Mutex
    let m: Mutex<i32> = Mutex::new(4);

    {
        // .lock() method actually unlocks the mutex
        // MutexGuard is a smart pointer (generally wrapped into a LockResult)
        let mut num: std::sync::MutexGuard<i32> = m.lock().unwrap();
        *num = 6;
    } //Auto release of the lock here because the MutexGuard goes out of scope


// ARC <=> ATOMIC REFERENCE COUNTER
// I have to go atomic in concurrency, Arc is much like Rc, but Atomic
// Arc is required for multiple threads to share a mutex

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10{
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move | | {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Counter after 10 counts: {}", *counter.lock().unwrap());

    // Note: I can create deadlocks

}
