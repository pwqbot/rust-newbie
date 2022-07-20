use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn five() -> i32 {
    return 5;
}

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number");
    println!("The secret_number is: {secret_number}");

    loop {
        println!("Please input your guess");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("hit!");
                break;
            }
            Ordering::Greater => println!("Too big"),
        }
    }


    let x = 5;
    let y = 5u8;

    let z = x + y;
    let tup = (500, 1.2, 1);
    // x = 6;
    const wd: u32 = 60 * 60 * 3;

    let condition = true;
    let number = if condition { 5 } else { 8 };


    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("23r21q34");

    for number in (1..4) {
        println!("{number}!");
    }
}
