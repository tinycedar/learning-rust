use std::thread;
use std::time::Duration;

fn test() {
    println!("in main thread");

    let _ = thread::spawn(|| {
        thread::sleep(Duration::from_millis(100));
        println!("in a thread");
    })
    .join();
}

fn test_move() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // println!("In main: {:?}", v); value borrowed here after move

    handle.join().unwrap();
}

mod tests {
    use super::*;

    #[test]
    fn test_thread() {
        test();
        test_move();
    }
}
