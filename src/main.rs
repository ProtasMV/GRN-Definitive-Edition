use std::{cmp::Ordering, io::{self, Write}};
use rand::random_range;
use colored::*;
use rand::prelude::IndexedRandom;

const SYS_COLOR: Color = Color::TrueColor { r: 244, g: 208, b: 63 };
const BG_COLOR: Color = Color::TrueColor { r: 160, g: 135, b: 40 };
const CORRECT_COLOR: Color = Color::TrueColor { r: 98, g: 168, b: 57 };
const WRONG_COLOR: Color = Color::Red;

const RANDOM_MIN: i32 = 1;
const EASY_RANDOM_MAX: i32 = 50;
const NORMAL_RANDOM_MAX: i32 = 100;
const HARD_RANDOM_MAX: i32 = 1000;
const UNPOSIBLE_RANDOM_MAX: i32 = 10000;
    
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
            "1"|"p"|"play" => {
                difficult();
                break
            },
            "2"|"d"|"description"|"des" => {
                description();
            },
            "3"|"q"|"quit"|"exit" => {
                break
            },
            _=> {
                println!("Invalid input, please try again");
                println!();
            }
        }
    }
}

fn difficult() {
    loop {
        println!();
        
        println!("Select difficulty");
        println!("1)Easy, {RANDOM_MIN}-{EASY_RANDOM_MAX}. 2)Normal, {RANDOM_MIN}-{NORMAL_RANDOM_MAX}. 3)Hard, {RANDOM_MIN}-{HARD_RANDOM_MAX}.");
        println!();
        println!("4)Unposible, {RANDOM_MIN}-{UNPOSIBLE_RANDOM_MAX} + disable Higher/Lower tips.");
        println!("5)Custom mode, You can choose the range and turn the hints on/off.");
        
        let (user_difficult, error) = input();
        if error {break}

        match user_difficult.as_str().trim() {
            "1" => {rand_num(RANDOM_MIN, EASY_RANDOM_MAX, true)},
            "2" => {rand_num(RANDOM_MIN, NORMAL_RANDOM_MAX, true)},
            "3" => {rand_num(RANDOM_MIN, HARD_RANDOM_MAX, true)},
            "4" => {rand_num(RANDOM_MIN, UNPOSIBLE_RANDOM_MAX, false)},
            "5" => {
                let (custom_min_range, custom_max_range, error) = loop {
                    println!();
                    let (error, custom_min_range) = custom_mode_num("Minimum");
                    if error {break (0, 0, true)}

                    let (error, custom_max_range) = custom_mode_num("Maximum");
                    if error {break (0, 0, true)}
                    
                    if custom_max_range < custom_min_range {
                        println!("Invalid values, maximum value cannot be less than minimum");
                        continue
                    } else {
                        break (custom_min_range, custom_max_range, false)
                    }
                };
                if error {break}
                
                let (tips, error) = tips_swith();
                if error {break}

                rand_num(custom_min_range, custom_max_range, tips);
            },
            _ => {println!("Invalid input, please try again"); continue}
        }
        break
    }
}

fn rand_num(rand_min: i32, rand_max: i32, tips: bool) {   
    'reset: loop {
        let mut best_attempt = u32::MAX;
        
        for round in 1..999999999 {
            let rand_number = random_range(rand_min..=rand_max);
            
            let (error, attempt) = main_game(rand_number, 0, tips, rand_min, rand_max);
            if error {break 'reset}

            if attempt < best_attempt {
                best_attempt = attempt
            }

            win(attempt, round, best_attempt);
            if !user_continue() {break 'reset};
        }
    }
}

fn main_game(rand_number: i32, mut attempt: u32, tips: bool, rand_min: i32, rand_max: i32) -> (bool, u32)  { 
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
        if error {println!("Invalid input, tip: try write number"); continue}
        if user_crossed_border(user_try, rand_min, rand_max) {
            continue
        }

        match user_try.cmp(&rand_number) {
            Ordering::Less => {
                tip(tips, "smaller");
                attempt += 1;
                continue
            },
            Ordering::Greater => { 
                tip(tips, "bigger");
                attempt += 1;
                continue
            },
            Ordering::Equal => {        
                attempt += 1;
                break (false, attempt);
            }
        }
    }
}

fn user_crossed_border(user_try: i32, rand_min: i32, rand_max: i32) -> bool {
    if user_try > rand_max || user_try < rand_min {
        println!("Actually, we agreed to play within a ({rand_min}-{rand_max}) radius, attempt not counted");
        true
    } else {
        false
    }
}

fn tip(tips: bool, typ: &str) {
    if !tips {
        println!("{}", "Incorrect!".color(WRONG_COLOR));
    } else {
        if typ == "bigger" {
            println!("{}", "Lower!".color(WRONG_COLOR))
        } else if typ == "smaller"{
            println!("{}", "Higher!".color(WRONG_COLOR))
        }
    }    
}

fn win(attempt: u32, round: u32, best_attempt: u32) {
    println!();
    
    let rank = rank_sys(attempt);
    println!("{rank}"); 

    if round >= 2 { 
        print!("{}", "Best score: ".color(SYS_COLOR));
        println!("{best_attempt}");
    }

    print!("{}", "Attempt: ".color(BG_COLOR));
    println!("{attempt}");
}

fn custom_mode_num(max_or_min: &str) -> (bool, i32) {
    let (error, cast_num) = loop {
        print!("{max_or_min} number: ");
        flush();

        let (cast_num, error) = input();
        if error {break (true, 0)}

        let (cast_num, error) = parse_to_i32(cast_num);
        if error {println!("Invalid input, tip: try write number"); continue}
        
        break (false, cast_num)
    };
    
    (error, cast_num)
}

fn tips_swith() -> (bool, bool) {
    let (tips, error) = loop {
        print!("Enable tips? (Y/N):");
        flush();

        let (tips, error) = input();
        if error {break (true, true)}

        let tips = match tips.trim().to_lowercase().as_str() {
            "y"|"yes"|"enable" => true,
            "n"|"no"|"disable" => false,
            _ => continue
        };

        break (tips, false)
    };
    (tips, error)
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
        Ok(_) => {return (data, false)},
        Err(er) => {println!("Error: {er}"); return (data, true)}
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

fn rank_sys(attempt: u32) -> String {
    let mut rng = rand::rng();
    let ranks: &[&str] = match attempt {
        0 => 
            &[
                "You won, but how?!",
                "phrase #2",
                "phrase #3",
                "phrase #4",
                "phrase #5"
            ],
        1 => 
            &[
                "You won! I don't even know how you did it!",
                "phrase #2",
                "phrase #3",
                "phrase #4",
                "phrase #5"
            ],
        2..=3 => 
            &[
                "You won! Richard the cat is proud of your skill!",
                "phrase #2",
                "phrase #3",
                "phrase #4",
                "phrase #5"
            ],
        4..=7 => &[
                "You won! Brilliant strategy and great intuition!",
                "phrase #2",
                "phrase #3",
                "phrase #4",
                "phrase #5"
            ],
        8..=12 => &[
                "You won! Solid result, steady and precise",
                "phrase #2",
                "phrase #3",
                "phrase #4",
                "phrase #5"
            ],
        13..=15 => &[
                "You won! Not a bad result, but Richard knows you can do better",
                "phrase #2",
                "phrase #3",
                "phrase #4",
                "phrase #5"
            ],
        994 | 993 => &[
                "You won! But wait... easter egg! (EASTER_EGG (1/5)",
                "You won! But wait... easter egg! (EASTER_EGG (2/5)",
                "You won! But wait... easter egg! (EASTER_EGG (3/5)",
                "You won! But wait... easter egg! (EASTER_EGG (4/5)",
                "You won! But wait... easter egg! (EASTER_EGG (5/5)"
            ],
        _ => &[
               "You won! Great job",
                "phrase #2",
                "phrase #3",
                "phrase #4",
                "phrase #5"
            ]
    };
    ranks.choose(&mut rng).expect("Random ranks error").to_string()
}