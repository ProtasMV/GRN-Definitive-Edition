use std::{cmp::Ordering, io::{self, Write}};
use rand::random_range;

fn main() {
    println!("
 _______  ______  __    _ 
|       ||    _ ||  |  | |
|    ___||   | |||   |_| |
|   | __ |   |_|||       |
|   ||  ||    __||  _    |
|   |_| ||   |\\\\ | | |   |
|_______||___| \\\\|_|  |__|
    ");

    println!("Welcome to Guess Random Number DFE!");
    loop {
        println!("1)Play, 2)Description");

        print!("Mode: ");
        flush();

        let (user_choise, error) = input();
        if error {break}

        match user_choise.trim().to_lowercase().as_str() {
            "1"|"play" => {
                main_game();
            },
            "2"|"description"|"des" => {

            }
            _=> {
                println!("Invalid input, please try again");
                println!();
            }
        }
    }
}

fn main_game() {
    loop {
        let rand_number = random_range(1..500);
        
        println!();        
        print!("Enter your guess: ");
        flush();
        let (user_try, error) = input();
        if error {break}

        let (user_try, error) = parse_to_i32(user_try);
        if error {continue}

        match user_try.cmp(&rand_number) {
            Ordering::Less => {println!("Bigger"); continue},
            Ordering::Greater => {println!("Smaller"); continue},
            Ordering::Equal => win()
        }
    }
}

fn win() {
    println!("");
    println!("You win!")
}

fn input() -> (String, bool) {
    let mut data = String::new();
    match io::stdin().read_line(&mut data) {
        Ok(_) => {return (data, false);},
        Err(er) => {println!("Error: {er}"); return (data, true);}
    }
}

fn flush() -> bool {
    match io::stdout().flush() {
        Ok(_) => {false}
        Err(er) => {println!("Error: {er}"); true}
    }
}

fn parse_to_i32(data: String) -> (i32, bool) {
    match data.trim().parse() {
        Ok(num) => (num, false),
        Err(er) => {println!("Error: {er}"); (0, true)}
    }
}