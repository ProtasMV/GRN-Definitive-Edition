use std::{cmp::Ordering, io::{self, Write}};
use rand::random_range;
use colored::*;

const SYS_COLOR: Color = Color::TrueColor { r: 244, g: 208, b: 63 };
const BG_COLOR: Color = Color::TrueColor { r: 160, g: 135, b: 40 };
const CORRECT_COLOR: Color = Color::TrueColor { r: 57, g: 135, b: 11 };
const WRONG_COLOR: Color = Color::Red;

fn main() {
    println!("{}", "
 _______  ______  __    _ 
|       ||    _ ||  |  | |
|    ___||   | |||   |_| |
|   | __ |   |_|||       |
|   ||  ||    __||  _    |
|   |_| ||   |\\\\ | | |   |
|_______||___| \\\\|_|  |__|
    ".color(SYS_COLOR));
    
    println!("{}", "Welcome to Guess Random Number DFE!".color(SYS_COLOR));
    println!("{}", "Made by: @ProtasMV".color(BG_COLOR));
    println!();
    
    loop {
        println!("{}", "1)Play, 2)Description".color(SYS_COLOR));

        print!("{}", "Mode: ".color(BG_COLOR));
        flush();

        let (user_choise, error) = input();
        if error {break}

        match user_choise.trim().to_lowercase().as_str() {
            "1"|"play" => {
                rand_num();
                break;
            },
            "2"|"description"|"des" => {
                description();
            }
            _=> {
                println!("Invalid input, please try again");
                println!();
            }
        }
    }
}

fn rand_num() {   
    'reset: loop {
        let mut best_attempt = 4294967295;
        
        for round in 1..999999999 {
            let rand_number = random_range(1..=500);
            let attempt: u32 = 0; 
            
            let (error, attempt) = main_game(rand_number, attempt);
            if error {break 'reset}

            if attempt < best_attempt {
                best_attempt = attempt
            }

            win(attempt, round, best_attempt);
            if !user_continue() {break 'reset};
        }
    }
}

fn main_game(rand_number: i32, mut attempt: u32) -> (bool, u32)  { 
    loop {
        println!();        
        
        //cheats
        print!("{}", "CHEATS: ".color(BG_COLOR));
        println!("{rand_number}");
        
        print!("{}", "Enter your guess: ".color(SYS_COLOR));
        flush();
        let (user_try, error) = input();
        if error {break (true, 0)}

        let (user_try, error) = parse_to_i32(user_try);
        if error {continue}

        match user_try.cmp(&rand_number) {
            Ordering::Less => {println!("{}", "Bigger".color(WRONG_COLOR)); attempt += 1; continue},
            Ordering::Greater => {println!("{}", "Smaller".color(WRONG_COLOR)); attempt += 1; continue},
            Ordering::Equal => {        
                attempt += 1;
                break (false, attempt);
            }
        }
    }
}

fn win(attempt: u32, round: u32, best_attempt: u32) {
    println!();
    println!("{}", "You win!".color(CORRECT_COLOR)); 

    if round >= 2 { 
        print!("{}", "Best score: ".color(SYS_COLOR));
        println!("{best_attempt}");
    }

    print!("{}", "Attempt: ".color(BG_COLOR));
    println!("{attempt}");
}

fn user_continue() -> bool {
    loop {
        println!();
        print!("{}", "Continue? (Y/N): ".color(SYS_COLOR));
        flush();

        let (user_want, error) = input();
        if error {return false}

        match user_want.trim().to_lowercase().as_str() {
            "1"|"y"|"yes" => break true,
            "2"|"n"|"no" => break false,
            _=> println!("Invalid input, please try again")
        }
    }
}

fn description() {
    println!("Not available in dev-beta");
    println!();
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