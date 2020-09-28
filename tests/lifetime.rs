use std::fmt::Display;

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_2<'a>(x: &'a str, y: &str) -> &'a str {
    println!("{}", y);
    x
}

fn smaller_size(x: &str, y: &str) -> usize {
    if x.len() > y.len() {
        x.len()
    } else {
        y.len()
    }
}

fn longest_with_an_announcement<'a, T: Display>(x: &'a str, y: &'a str, ann: T) -> &'a str {
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_lifetime() {
        let longest: &str = longest("1", "123");
        println!("{}", longest);
        println!("{}", longest_2("1", "123"));
        println!("{}", smaller_size("1", "123"));
        println!("{}", longest_with_an_announcement("a", "abc", 1));
    }
}
