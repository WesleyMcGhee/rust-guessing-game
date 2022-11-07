use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main () {
    println!("Guess the Number");
    // generates random number
    let hidden_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();
        // user input
        io::stdin()
            .read_line(&mut guess)
            .expect("falild to read line");

        // converts string to integer, and checks to make sure value is an integer
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        //matches the guess and handles result
        match guess.cmp(&hidden_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
