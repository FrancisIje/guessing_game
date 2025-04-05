use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        println!("Enter your guess");
        io::stdin().read_line(&mut guess).expect("Enter your guess");
        println!("Your guess is :{guess}");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("error: {err}",);
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("The number is small");
            }
            Ordering::Equal => {
                println!("You winnnnn");
                break;
            }
            Ordering::Greater => {
                println!("The number is biggg")
            }
        }
    }
}
