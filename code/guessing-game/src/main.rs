use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	// generate the secret number and allocate the guess
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Guess the number!");

    loop {
        let mut guess = String::new();
        println!("Please input your guess.");

        // read in the user's guess
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // cast the guess string to an int
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

        // compare the guess with the secret
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
