// Did not know this comes from a children's game https://en.wikipedia.org/wiki/Fizz_buzz
use num_traits::{identities::Zero, PrimInt}; // 0.2.14

pub trait Fizzy {
    fn fizzy(&self) -> String;
}

impl<T> Fizzy for T
where
    T: PrimInt + Zero,
    T: std::fmt::Display,
{
    fn fizzy(&self) -> String {
        let zero = T::zero();
        let three = T::from(3).unwrap(); // These will never fail
        let five = T::from(5).unwrap();
        match (*self % three, *self % five) {
            (x, y) if x == zero && y == zero => String::from("FizzBuzz"),
            (x, _) if x == zero => String::from("Fizz"),
            (_, x) if x == zero => String::from("Buzz"),
            _ => format!("{}", self),
        }
    }
}

fn main() {
    for (xi32, xu8) in (1..=30).zip(1u8..) {
        println!("{}\t{}", xi32.fizzy(), xu8.fizzy())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fizz() {
        for x in &[3, 6, 27] {
            assert_eq!(x.fizzy(), "Fizz")
        }
    }

    #[test]
    fn test_buzz() {
        for x in &[5, 10, 20] {
            assert_eq!(x.fizzy(), "Buzz")
        }
    }

    #[test]
    fn test_fizzbuzz() {
        for x in &[15, 30, 60] {
            assert_eq!(x.fizzy(), "FizzBuzz")
        }
    }

    #[test]
    fn test_num() {
        for x in &[7,13, 29] {
            assert_eq!(x.fizzy(), format!("{}",x))
        }
    }
}
