// Did not know this comes from a children's game https://en.wikipedia.org/wiki/Fizz_buzz
fn fizzbuzz(num: i32) -> String {
    match (num % 3, num % 5) {
        (0, 0) => String::from("fizzbuzz"),
        (0, _) => String::from("fizz"),
        (_, 0) => String::from("buzz"),
        (_, _) => format!("{}", num),
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
        assert_eq!(fizzbuzz(36), String::from("fizz"))
    }

    #[test]
    fn test_buzz() {
        assert_eq!(fizzbuzz(25), String::from("buzz"))
    }

    #[test]
    fn test_fizzbuzz() {
        assert_eq!(fizzbuzz(30), String::from("fizzbuzz"))
    }
}
