// Did not know this comes from a children's game https://en.wikipedia.org/wiki/Fizz_buzz
use core::fmt;
use std::fmt::Formatter;

// Rust enums allow for contained values (this is exactly what Result and Option do)
// Fizzbuzz is an opportunity to show it, since the num case can contain the actual value
#[derive(Debug, PartialEq)]
enum FizzBuzz {
    Fizz,
    Buzz,
    FizzBuzz,
    Num(i32),
}

// Implementing display lets us print the result
impl fmt::Display for FizzBuzz {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FizzBuzz::Fizz => write!(f, "fizz"),
            FizzBuzz::Buzz => write!(f, "buzz"),
            FizzBuzz::FizzBuzz => write!(f, "fizzbuzz"),
            FizzBuzz::Num(val) => write!(f, "{}", val),
        }
    }
}

impl From<i32> for FizzBuzz {
    fn from(num: i32) -> Self {
        match (num % 3, num % 5) {
            (0, 0) => FizzBuzz::FizzBuzz,
            (0, _) => FizzBuzz::Fizz,
            (_, 0) => FizzBuzz::Buzz,
            _ => FizzBuzz::Num(num),
        }
    }
}

fn main() {
    for x in 1..101 {
        println!("{}", FizzBuzz::from(x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizz() {
        assert_eq!(FizzBuzz::from(36), FizzBuzz::Fizz)
    }

    #[test]
    fn test_buzz() {
        assert_eq!(FizzBuzz::from(25), FizzBuzz::Buzz)
    }

    #[test]
    fn test_fizzbuzz() {
        assert_eq!(FizzBuzz::from(30), FizzBuzz::FizzBuzz)
    }

    #[test]
    fn test_num() {
        assert_eq!(FizzBuzz::from(7), FizzBuzz::Num(7))
    }
}
