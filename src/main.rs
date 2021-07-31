use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number! ");

    let secrete_number = rand::thread_rng().gen_range(1..101);
    println!("The secrete number is : {}.", secrete_number);

    println!("Please input your guess: ");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!!");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    
    println!("You guessed {}", guess);

    match guess.cmp(&secrete_number){
        Ordering::Less => println!("Your guess is smaller than our our number"),
        Ordering::Greater => println!("Your guess is greater than our our number"),
        Ordering::Equal => println!("Congratulations!! You got the number!"),
    }

}
