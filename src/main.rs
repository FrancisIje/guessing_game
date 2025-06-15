use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut guess_number = String::new();
        println!("Enter a number from 1-100");
        match io::stdin().read_line(&mut guess_number) {
            Err(err) => {
                println!("{err}");
            }
            Ok(_) => {
                println!("You entered {guess_number}");
            }
        };
        let guess_number: u32 = match guess_number.trim().parse() {
            Ok(n) => n,
            Err(err) => {
                println!("{err}");
                continue;
            }
        };
        match guess_number.cmp(&secret_number) {
            Ordering::Equal => {
                println!("Wow you got it right");
                break;
            }
            Ordering::Greater => {
                println!("Hmmm a little too big");
            }
            Ordering::Less => {
                println!("Hmmm a little too small");
            }
        }
    }
}
