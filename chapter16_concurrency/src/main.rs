/**
 * 1. Concurrency in rust:
 *      - Only supports native thread (system thread or os thread), it does not support grean thread
 *      - Mean each thread is runing independently and simutenously.
 * 2. Using Thread:
 *      - std lib provides tread namespace to create a new thread -> Ref1
 *      - each thread.spawn() returns a a JoinHandler type. we can call .join() method to block the thread that is currently runing (main) untill the thread associated with handler (spawn) terminates. -> Ref2
 * 
 */

fn main() {
                                                                        // Ref1
    // {
    //     use std::thread;
    //     use std::time::Duration;
    //     thread::spawn( || {
    //         for i in 1..10 {
    //             println!("Count for new thread -> {}", i);
    //             thread::sleep(Duration::from_millis(1000))
    //         }
    //     });
    //     for i in 1..5 {
    //         println!("Count for main thread -> {}", i);
    //         thread::sleep(Duration::from_millis(1000))
    //     }                                                               // We will see that the newly created thread is terminated when main thread is finished
    // }
                                                                        // Ref2
    {
        use std::thread;
        use std::time::Duration;
        let handler = thread::spawn( || {
            for i in 1..10 {
                println!("Count for new thread -> {}", i);
                thread::sleep(Duration::from_millis(1000))
            }
        });
        for i in 1..5 {
            println!("Count for main thread -> {}", i);
            thread::sleep(Duration::from_millis(1000))
        }      
        handler.join().expect("Fail to join thread")                    // join() method returns a Result
    }
}
