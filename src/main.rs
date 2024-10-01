use rand :: Rng;
//This will help in generating random number by importing rand library
use std::io;
//This is standard I/O library
use std::cmp::Ordering;
//This is standard Ordering Library used to compare

fn main() {
    println!("Welcome to the Guessing Game");
    let secret_number = rand::thread_rng().gen_range(1..11);
    //101 is exclusive and 1 is inclusive
    // println!("Secret Number is {}", secret_number);  //Used this just to understand the concept

    loop {
        println!("Please Input Your Guess(1-10)");

    let mut guess = String::new();  //This will take number but as a string

    io::stdin().read_line(&mut guess) 
        .expect("You entered a number which is out of scope.");      //Will catch errors

    println!("Your guess is {}", guess);

    //Now to convert the string to a number
    let guess:u32 = guess.trim().parse()
        .expect("Please Type an Integer");      //converts string to integer
    //trim will remove the spaces between the string

    // println!("{}", guess+1);  Used this to check if no converted to number from string

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Value is smaller than the secret value"),
        Ordering::Greater => println!("Value is greater than the secret value"),
        Ordering::Equal => println!("You guessed {} which is correct bro.", guess)
    }
    }
}
