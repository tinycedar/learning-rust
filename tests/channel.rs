mod tests {
    use std::ops::Add;
    use std::sync::mpsc;
    use std::thread;

    #[test]
    fn test_channel() {
        let (tx1, rx) = mpsc::channel();
        let tx2 = mpsc::Sender::clone(&tx1);

        thread::spawn(move || {
            for i in 0..4 {
                tx1.send(String::from("hi => ").add(i.to_string().as_str()))
                    .unwrap();
            }
        });

        thread::spawn(move || {
            tx2.send(String::from("other")).unwrap();
        });

        loop {
            match rx.recv() {
                Ok(str) => println!("{}", str),
                Err(_) => {
                    // println!("none: {}", rec_err.to_string());
                    break;
                }
            }
        }
    }
}
