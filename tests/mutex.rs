use std::sync::Mutex;

fn mutex() {
    let m = Mutex::new(5);
    let mut num = m.lock().unwrap();
    *num = 1;
    println!("{}", num);
}

mod tests {
    use super::*;

    #[test]
    fn test_mutex() {
        mutex();
    }
}
