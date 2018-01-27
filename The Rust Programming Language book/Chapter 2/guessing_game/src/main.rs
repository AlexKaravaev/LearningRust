extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	println!("Guess the number!");

	let secret_number = rand::thread_rng().gen_range(1,101);

    
    loop {
	    println!("Please input your guess.");

	    /*
	    * let foo = 5; 		--- immutable
	    * let mut bar = 5;	--- mutable
	    */

	    let mut guess = String::new(); //::new() just returns empty instance of String

	    /*
	    * read_line returns enum with 2 values: Ok and Err
	    * If instance is Err, then expect will cause programm to crash
	    */

	    io::stdin().read_line(&mut guess) 
	    	.expect("Failed to read line");

	    /*
	    * trim() delets all whitespaces in String
	    * parse() returns number on parsed tring
	    */

	    let guess: u32 = match guess.trim().parse()
		{
	    	Ok(num) => num,
	    	Err(_) => continue,
	    };

	    println!("You guessed: {}",guess);


	    /* cmp returns Ordering instance 
	    * (Ordering is enum with Less Greater Equal vals)
	    * match expression allows us to decide what to do
	    * based on what cmp has returned
	    */

	    match guess.cmp(&secret_number) {
	    	Ordering::Less => println!("Too Small!"),
	    	Ordering::Greater => println!("Too big!"),
	    	Ordering::Equal => {
	    		println!("You win!");
	    		break;
	    	}
	    } 
    }

}
