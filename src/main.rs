use std::io; // bringing this into scope from the standard library
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number : i16 = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut guess = String::new();
        println!("Please input your guess.");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess : i16 = match guess
                            .trim()
                            .parse() {
                                Ok(num) => num, 
                                Err(_) => continue,
                            };
        println!("this is your guess:  {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }

    println!("this is the secret:  {}", secret_number);


}
