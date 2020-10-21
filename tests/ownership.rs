mod tests {
    #[test]
    fn test_move() {
        let s = String::from("hello");
        let t = s;

        println!("{}", t);
        let x = t;
        println!("{}", x);
    }

    #[test]
    fn test_borrow() {
        let mut s = String::from("hello");
        let t = &s;
        println!("{}", t);

        let x = &mut s;
        x.push_str(" world");
        println!("{}", x);
    }
}