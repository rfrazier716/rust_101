use std::cmp::Ordering;
use std::io::Write;
use rand::Rng; //used for the random number generation

fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}: ",prompt);
    std::io::stdout().flush().expect("could not flush output buffer"); // flush the output to make sure the line is seen
    std::io::stdin().read_line(&mut input).expect("Could not read input");
    input
}

fn get_guess() -> i32 {
    let guess: String = get_input("Guess a number");
    guess.trim().parse().expect("Could not derive a number from guess")
}

fn compare_guess(guess: i32, actual: i32) -> bool{
    let mut guessed_correctly = false;

    // compare the user's guess to the secret number
    match guess.cmp(&actual){
        Ordering::Less => {println!("Your guess was too small")}
        Ordering::Equal => {
            println!("You got it right!");
            guessed_correctly = true;}
        Ordering::Greater => {println!("Your guess was too big")}
    };
    guessed_correctly // a boolean on if the guess was right or not
}

fn main() {
    const MIN:i32 = 1;
    const MAX:i32 = 100;
    println!("Can you guess the secret Number??? ({}-{})", MIN, MAX);

    let secret_number:i32 = rand::thread_rng().gen_range(MIN, MAX); // randomly create a secret number
    let mut user_guess = secret_number + 1; //intentionally mismatch the secret number from the guess
    let mut num_guesses = 0; //count how many guesses the user has made

    while user_guess!=secret_number{
        user_guess = get_guess(); // Get the guess from the user
        compare_guess(user_guess, secret_number);
        num_guesses += 1;
    }

    println!("You won the game after {} guesses", num_guesses);
}
