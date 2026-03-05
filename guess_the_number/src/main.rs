use std::io;
use rand::prelude::*;

fn main() {
    let mut rng = rand::rng();

    let gen_number: i64 = rng.random_range(0..=100);

    

    loop {
        println!("Enter a number from 1 to 100: ");
        let user_input: i64 = get_int();
        if user_input == gen_number {
            break println!("You guessed it!")
        } else if user_input < gen_number {
            println!("Too low!")
        } else if user_input > gen_number {
            println!("too high!")
        }
    }   
}

// fn get_string() -> String {
//     loop {
//         let mut s: String = String::new();
//         match io::stdin().read_line(&mut s) {
//             Ok(_) => return s.trim().to_string(),
//             Err(e) => println!("Failed to read input: {}", e)
//         }
//     }
// }

fn get_int() -> i64 {
    loop {
        let mut s: String = String::new();
        match io::stdin().read_line(&mut s) {
            Ok(_) =>  match s.trim().parse::<i64>() {
                Ok(n) => return n,
                Err(e) => println!("Failed to convert to integer: {}", e)
            }
            Err(e) => println!("Failed to read input: {}", e)
        }
    }
}
