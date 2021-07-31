//Guess the number game written Rust 
//Rust::Systems programming language for the 21st century
/**
    Source::RustDocs url::https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
 */

 //importing modules
use std::io;
use std::cmp::Ordering;
use rand::Rng;

//the main function of rust program
fn main() {
    //printing to the console
    println!("Guess the number! ");

    //declaring variables
    let secrete_number = rand::thread_rng().gen_range(1..101);
    
    println!("Please input your guess: ");

    //declaring mutable variables
    let mut guess = String::new();

    //reading user input
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!!");

    //making sure that our first guess & 2nd guess is of the same datatype (small int )
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    
    println!("You guessed {}", guess);

    //Doing comparison using cmp module from the standard library
    match guess.cmp(&secrete_number){
        Ordering::Less => println!("Your guess is smaller than our our number"),
        Ordering::Greater => println!("Your guess is greater than our our number"),
        Ordering::Equal => println!("Congratulations!! You got the number!"),
    }

    println!("The secrete number is : {}.", secrete_number);
    
    //It was fun doing this am in LOVE with RUST already! Though i'm not sure if i'll ever use it somewhere in a real project
}
