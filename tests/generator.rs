#![feature(generators, generator_trait)]

use std::ops::Generator;
use std::pin::Pin;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator() {
        let mut generator = || {
            println!("2");
            yield;
            println!("4");
        };

        println!("1");
        Pin::new(&mut generator).resume(());
        println!("3");
        Pin::new(&mut generator).resume(());
        println!("5");
    }
}
