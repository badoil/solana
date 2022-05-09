use std::io;
use std::cmp::Ordering;
use rand::Rng; 
use colored::*; 

fn main() {
    println!("guess the number");

    let secret_number: u32 = rand::thread_rng().gen_range(1, 101);
    print!("secret_number number is {}", secret_number );

    loop {
        println!("please input your guess");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        print!("guess number is {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => print!("{}", "less".red()),
            Ordering::Greater => print!("{}", "Greater".red()),
            Ordering::Equal => {
                print!("{}", "Win".green());
                break;
            }
        }
    }
 
}
