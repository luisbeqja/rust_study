use std::{ io, cmp::Ordering };
use rand::Rng;
use colored::*;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("the number is {}", secret_number);

    loop {
        let mut guess: String = String::new();

        io::stdin().read_line(&mut guess).expect("error");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };

        println!("You souggest: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small".red()),
            Ordering::Greater => println!("{}", "Too big".red()),
            Ordering::Equal => {
                println!("{}", "You winnn".green());
                break;
            }
        }
    }
}
