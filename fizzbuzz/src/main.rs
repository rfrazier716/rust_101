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
// They all need to be strings, since calling format! on val has unknown stack size at compile time
impl fmt::Display for FizzBuzz {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let display = match self {
            FizzBuzz::Fizz => String::from("fizz"),
            FizzBuzz::Buzz => String::from("buzz"),
            FizzBuzz::FizzBuzz => String::from("fizzbuzz"),
            FizzBuzz::Num(val) => format!("{}", val),
        };
        write!(f, "{}", display)
    }
}

fn fizzbuzz(num: i32) -> FizzBuzz {
    match (num % 3, num % 5) {
        (0, 0) => FizzBuzz::FizzBuzz,
        (0, _) => FizzBuzz::Fizz,
        (_, 0) => FizzBuzz::Buzz,
        (_, _) => FizzBuzz::Num(num),
    }
}

fn main() {
    for x in 1..101 {
        println!("{}", fizzbuzz(x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizz() {
        assert_eq!(fizzbuzz(36), FizzBuzz::Fizz)
    }

    #[test]
    fn test_buzz() {
        assert_eq!(fizzbuzz(25), FizzBuzz::Buzz)
    }

    #[test]
    fn test_fizzbuzz() {
        assert_eq!(fizzbuzz(30), FizzBuzz::FizzBuzz)
    }

    #[test]
    fn test_num() {
        assert_eq!(fizzbuzz(7), FizzBuzz::Num(7))
    }
}
