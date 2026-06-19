use std::{cmp::Ordering, io::{self, Write}};
use rand::random_range;
use owo_colors::*;
use rand::prelude::IndexedRandom;

const SYS_COLOR: Style = Style::new().fg_rgb::<244, 208, 63>();
const BG_COLOR: Style = Style::new().fg_rgb::<160, 135, 40>();
const CORRECT_COLOR: Style = Style::new().fg_rgb::<98, 168, 57>();
const WRONG_COLOR: Style = Style::new().red();

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
    ".style(SYS_COLOR));
    
    println!("{}", "Welcome to Guess Random Number DFE!".style(SYS_COLOR));
    println!("{}", "Made by: @ProtasMV".style(BG_COLOR));
    println!();
    
    loop {
        println!("{}", "1)Play, 2)Description, 3)Exit".style(SYS_COLOR));

        print!("{}", "Mode: ".style(BG_COLOR));
        if flush().is_err() {break}

        let Ok(user_choise) = input() else {break};

        match user_choise.trim().to_lowercase().as_str() {
            "1"|"p"|"play" => {
                difficult();
            },
            "2"|"d"|"description"|"des" => {
                description();
            },
            "3"|"e"|"exit"|"quit" => {
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
        
        println!("{}", "Select difficulty".style(SYS_COLOR));
        println!("{}", format!("1)Easy, {RANDOM_MIN}-{EASY_RANDOM_MAX}. 2)Normal, {RANDOM_MIN}-{NORMAL_RANDOM_MAX}. 3)Hard, {RANDOM_MIN}-{HARD_RANDOM_MAX}").style(SYS_COLOR));
        println!();
        println!("{}", format!("4)Unposible, {RANDOM_MIN}-{UNPOSIBLE_RANDOM_MAX} + disable Higher/Lower tips").style(SYS_COLOR));
        println!("{}", format!("5)Custom mode, You can choose the range and turn the hints on/off").style(SYS_COLOR));
        
        print!("{}", "Mode: ".style(BG_COLOR));
        if flush().is_err() {break}
        
        let Ok(user_difficult) = input() else {break};

        match user_difficult.as_str().trim() {
            "1" => {rand_num(RANDOM_MIN, EASY_RANDOM_MAX, true)},
            "2" => {rand_num(RANDOM_MIN, NORMAL_RANDOM_MAX, true)},
            "3" => {rand_num(RANDOM_MIN, HARD_RANDOM_MAX, true)},
            "4" => {rand_num(RANDOM_MIN, UNPOSIBLE_RANDOM_MAX, false)},
            "5" => {
                let custom_range = loop {
                    println!();
                    let Ok(custom_min_range) = custom_mode_num("Minimum") else {break Err(())};
                    let Ok(custom_max_range) = custom_mode_num("Maximum") else {break Err(())};
                    
                    if custom_num_check(custom_min_range, custom_max_range) {
                        break Ok((custom_min_range, custom_max_range))
                    }
                };
                let Ok((custom_min_range, custom_max_range)) = custom_range else {break};
                let Ok(tips) = tips_swith() else {break};

                rand_num(custom_min_range, custom_max_range, tips);
            },
            _ => {println!("Invalid input, please try again"); continue}
        }
        break
    }
}

fn custom_num_check(custom_min_range: i32, custom_max_range: i32) -> bool {
    if custom_max_range < custom_min_range {
        println!("Invalid values, maximum value cannot be less than minimum");
        false
    } else {
        true
    }    
}

fn rand_num(rand_min: i32, rand_max: i32, tips: bool) {   
    'reset: loop {
        let mut best_attempt = u32::MAX;
        
        for round in 1..999999999 {
            let rand_number = random_range(rand_min..=rand_max);
            let Ok(attempt) = main_game(rand_number, 0, tips, rand_min, rand_max) else {break 'reset};

            if attempt < best_attempt {
                best_attempt = attempt
            }

            win(attempt, round, best_attempt);
            if !user_continue() {break 'reset};
        }
    }
}

fn main_game(rand_number: i32, mut attempt: u32, tips: bool, rand_min: i32, rand_max: i32) -> Result<u32, ()>  { 
    loop {
        println!();        
        
        //cheats
        print!("{}", "CHEATS: ".style(BG_COLOR));
        println!("{rand_number}");
        
        print!("{}", "Enter your guess: ".style(SYS_COLOR));
        if flush().is_err() {break Err(())}
        let Ok(user_try) = input() else {break Err(())};

        let Ok(user_try) = parse_to_i32(user_try) else {
            println!("Invalid input, tip: try write number");
            continue};
            
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
                break Ok(attempt);
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
        println!("{}", "Incorrect!".style(WRONG_COLOR));
    } else {
        if typ == "bigger" {
            println!("{}", "Lower!".style(WRONG_COLOR))
        } else if typ == "smaller"{
            println!("{}", "Higher!".style(WRONG_COLOR))
        }
    }    
}

fn win(attempt: u32, round: u32, best_attempt: u32) {
    println!();
    
    let rank = rank_sys(attempt);

    println!("{}", rank.style(CORRECT_COLOR)); 

    if round >= 2 { 
        print!("{}", "Best score: ".style(SYS_COLOR));
        println!("{best_attempt}");
    }

    print!("{}", "Attempt: ".style(BG_COLOR));
    println!("{attempt}");
}

fn custom_mode_num(max_or_min: &str) -> Result<i32, ()> {
    loop {
        print!("{}", format!("{max_or_min} number: ").style(SYS_COLOR));
        if flush().is_err() {break Err(())}

        let Ok(cast_num) = input() else {break Err(())};

        let Ok(cast_num) = parse_to_i32(cast_num) else {
            println!("Invalid input, tip: try write number");
            continue
        };
        
        break Ok(cast_num)
    }
}

fn tips_swith() -> Result<bool, ()> {
    loop {
        print!("{}", "Enable tips? (Y/N):".style(SYS_COLOR));
        if flush().is_err() {break Err(())}

        let Ok(tips) = input() else {break Err(())};

        let tips = match tips.trim().to_lowercase().as_str() {
            "y"|"yes"|"enable" => true,
            "n"|"no"|"disable" => false,
            _ => continue
        };

        break Ok(tips)
    }
}

fn user_continue() -> bool {
    loop {
        println!();
        print!("{}", "Continue? (Y/N): ".style(SYS_COLOR));
        if flush().is_err() {return false}

        let Ok(user_want) = input() else {return false};

        match user_want.trim().to_lowercase().as_str() {
            "1"|"y"|"yes" => break true,
            "2"|"n"|"no" => {
                break false
            },
            _=> println!("Invalid input, please try again")
        }
    }
}

fn description() {
    println!("  The goal of this game is to improve upon the original Guessing Game from the official Rust tutorial (The Rust Book). 
    While the original code is about 30 lines long,
    the Definitive Edition version contains 340+ lines");
    println!();
}

fn input() -> Result<String, ()> {
    let mut data = String::new();
    match io::stdin().read_line(&mut data) {
        Ok(_) => {Ok(data)},
        Err(er) => {println!("Error: {er}"); return Err(())}
    }
}

fn flush() -> Result<(), ()> {
    match io::stdout().flush() {
        Ok(_) => {Ok(())}
        Err(er) => {println!("Error: {er}"); Err(())}
    }
}

fn parse_to_i32(data: String) -> Result<i32, ()> {
    match data.trim().parse() {
        Ok(num) => Ok(num),
        Err(er) => {println!("Error: {er}"); Err(())}
    }
}

fn rank_sys(attempt: u32) -> String {
    let mut rng = rand::rng();
    let ranks: &[&str] = match attempt {
        0 => 
            &[
                "You&6%&@#2.. 0 attempts? Okey, here's a def-fact:
                 a significant part of this code was written on Fedora Linux inside the KDE Plasma environment",
            ],
        1 => 
            &[
                "You won! I don't even know how you did it!",
                "You won! 1 attempt... Wait, did you just guess it or did a memory leak show you the number?",
                "You won! Weird 1 attempt, but... let's accept",
                "You won! 1 attempt, Richard is very surprised by your mastery, but at the same time doubts your honesty.",
                "You w... wait, already?"
            ],
        2..=3 => 
            &[
                "You won! Richard the cat is proud of your skill!",
                "You won! 2-3 attempts, pretty good, Richard the cat approves",
                "You won! Fast and clean, nice job",
                "You won! Richard is purring, so you definitely did well",
                "You won, That was actually impressive"
            ],
        4..=7 => &[
                "You won! Brilliant strategy and great intuition!",
                "You won, 4-7 attempts. A completely normal, good result",
                "You won! Richard is still watching you, keep it up",
                "You won! You're getting good at this",
                "You won, don't bad"
            ],
        8..=12 => &[
                "You won! Solid result, steady and precise",
                "You won! Took a few tries, but a win is a win",
                "You won, Richard the cat is getting a bit sleepy, but he saw you win",
                "You won, not perfect, but you got there in the end",
                "You won, maybe good job"
            ],
        13..=15 => &[
                "You won! Not a bad result, but Richard knows you can do better",
                "You won, but honestly, it shouldn't have taken this long",
                "You won! Richard is staring at you with pure disappointment",
                "You won, though it took you so long that Richard left the room out of boredom",
                "You won! Just please don't show anyone this attempt counter"
            ],
        994 | 993 => &[
                "An Easter egg was supposed to be here, but the author cut the Easter egg system.
                 Consider the very fact that they were even planned as an Easter egg in itself"
            ],
        _ => &[
                "You won! Great job",
                "You won, finally... because that was painful to watch, even the cat is disappointed",
                "You won! That attempt counter is just embarrassing",
                "You won, but are you just guessing every single number in order? It looks like it",
                "You won! I was genuinely starting to lose hope in you"
            ]
    };
    ranks.choose(&mut rng).expect("Random ranks error").to_string()
}