use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The larger member is x = {}", self.x);
        } else {
            println!("The larger member is y = {}", self.y);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_2() {
        let p = Pair::new(1, 2);
        println!("Pair x: {}", p.x);
        p.cmp_display();

        let pp = Pair::new(&p, &p);
        // pp.cmp_display(); trait bounds were not satisfied
    }
}
