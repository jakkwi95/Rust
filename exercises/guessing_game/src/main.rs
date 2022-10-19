use std::io;
use std::cmp::Ordering;

 // random is not in standard library so its needed to be added in dependencies file
use rand::Rng;

fn main() {
    println!("Welcome to the guessing game!");

    // using rand library to assign thread and get number from range 1-100
    let secret_number = rand::thread_rng().gen_range(1..=100); 

    println!("Try guessing the number I have picked!");

    loop {
        println!("Whats your quess?");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line.");


        // making shadowed variable of guess to check if its correct (digit not a letter or special signs)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{guess} is too low!"),
            Ordering::Greater => println!("{guess} is too high!"),
            Ordering::Equal => {
                println!("You win!"); 
                break;
            }
        }
    }
}
