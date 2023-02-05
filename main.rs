// import library
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");

    // generate random number from 1 to 100
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your guess.");

        //create new string to store value that is mutable
        let mut guess = String::new();

        // received input of the string and except to throw exception
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // parse the number convert from string to int
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

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

// function
// fn main() {
//     println!("Hello, world!");

//     another_function();
//     another_function2(5);
//     print_labeled_measurement(5, 'h');
// }

// fn another_function() {
//     println!("Another function.");
// }

// fn another_function2(x: i32) {
//     println!("The value of x is: {x}");
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }
