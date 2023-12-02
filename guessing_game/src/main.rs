// by
use std::io;
//to compare
use std::cmp::Ordering;

use rand::Rng;
//standard library that  contains io
fn main() {
    println!("Guess a number");

    // new mutable variable
    let mut guess = String::new();
    //String is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text.

    //The :: syntax in the ::new line indicates that new is an associated function of the String type.
    //An associated function is a function that?s implemented on a type

   
    // We create a variable named guess. But wait, doesn?t the program already have a variable named guess?
    //  It does, but helpfully Rust allows us to shadow the previous value of guess with a new one.
    // Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables, such as guess_str and guess

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");


        //convert the string to a number
        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        //calls stdin function
        //analog to std::io::stdin --> wich returns a istance of std::io::stdin

        // read_line calls on it the standard imput putting the variable for the input
        // & means is passed as a reference (a pointer)

        //expect handles the case of failure
        println!("guessed : {guess}");

        // cant process inside the curly only print
        println!("secret : {secret_number}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
