// Did not know this comes from a children's game https://en.wikipedia.org/wiki/Fizz_buzz
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
