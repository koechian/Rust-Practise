// 1. Generate a random number between 1-100
// 2. take in user input of a number
// 3. Return how far the guess was from the actual number (if it was more or less)
// 4. Loop until the correct number has been found


use rand::Rng;
use std::cmp::Ordering;


fn main() {
    
    let random_number = number_generator();
    
    loop{

    
    let user_guess = get_guess();
    match user_guess.cmp(&random_number){

        Ordering::Less => println!("Too small"),
        Ordering::Equal => {println!("You win"); break;},
        Ordering::Greater => println!("Too Large")

    }
}

}

fn get_guess() -> u32{

    println!("Guess the number Game!");
    println!("Please input yout guess...");

    let mut guess = String::new();

    std::io::stdin()
    .read_line(&mut guess)
    .expect("Failed to readline");

    let guess:u32 = guess.trim().parse().expect("Please input a number");

    return guess;

}

fn number_generator() -> u32{
    
    let num = rand::thread_rng().gen_range(0..100);

    num
}