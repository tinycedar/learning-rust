use std::fmt::{Display, Error, Formatter};

struct Duck;

struct Pig;

trait Fly {
    fn fly(&self) -> bool;
}

impl Fly for Duck {
    fn fly(&self) -> bool {
        true
    }
}

impl Display for Duck {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str(self.fly().to_string().as_str())
    }
}

impl Fly for Pig {
    fn fly(&self) -> bool {
        false
    }
}

impl Display for Pig {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str(self.fly().to_string().as_str())
    }
}

fn fly_static<T: Fly + Display>(s: T) -> bool {
    s.fly()
}

// syntax sugar for fly_static
fn fly_static_1(s: impl Fly + Display) -> bool {
    s.fly()
}

fn fly_static_2(s: &(impl Fly + Display)) -> bool {
    s.fly()
}

fn fly_static_3<T, U>(t: &T, u: &U) -> bool
where
    T: Fly + Display,
    U: Fly + Display,
{
    t.fly() && u.fly()
}

fn fly_dyn(s: &dyn Fly) -> bool {
    s.fly()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait() {
        let pig = Pig;
        assert_eq!(fly_static::<Pig>(pig), false);
        assert_eq!(fly_static_1(Pig {}), false);
        assert_eq!(fly_static_2(&Pig {}), false);
        assert_eq!(fly_static_3(&Pig {}, &Pig {}), false);

        let duck = Duck;
        assert_eq!(fly_dyn(&duck), true);
    }
}
