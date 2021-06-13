// Did not know this comes from a children's game https://en.wikipedia.org/wiki/Fizz_buzz
fn fizzbuzz(num: i32) -> String{
    String::from("Hello")
}

fn main() {
    for num in 1..101 {
        if (num % 3 == 0) && (num % 5 == 0){
            println!("fizzbuzz")
        }
        else if num % 3 == 0{
            println!("fizz")
        } else if num % 5 == 0{
            println!("buzz")
        } else {println!("{}",num)}
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_fizz(){
        assert_eq!(fizzbuzz(36), String::from("fizz"))
    }

    #[test]
    fn test_buzz(){
        assert_eq!(fizzbuzz(25), String::from("buzz"))
    }

    #[test]
    fn test_fizzbuzz(){
        assert_eq!(fizzbuzz(30), String::from("fizzbuzz"))
    }
}