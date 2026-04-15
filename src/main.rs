use std::cmp::Ordering;
use std::io;
use rand::Rng;
use colored::*;

fn main() {
    const LATUTA_EASTER: &str = "/latuta";
    const RICHARD_EASTER: &str = "/richard";
    const STOP_COMAND: &str = "/stop";
    const CHEATS_COMAND: &str = "/cheats";
    const RANDOM_MIN: u32 = 1;
    const EASY_RANDOM_MAX: u32 = 50;
    const NORMAL_RANDOM_MAX: u32 = 100;
    const HARD_RANDOM_MAX: u32 = 1000;
    const UNPOSIBLE_RANDOM_MAX: u32 = 10000;
    
    let mut trying = 0;
    let mut mode = String::new();
    let mut tips_off = 0;
    let mut best_trying_one = 999;
    let mut round = 1;
    let mut best_trying_two = 999;
    let mut best_trying_duo = 0;
    let mut best_result_time = 0;
    let mut setting_or_game = String::new();
    let mut return_to_menu = String::new();
    let mut cheats_on = 0;

    let mut castom_num_one = String::new();
    let mut castom_num_two = String::new();
    let mut castom = 0;
    let mut castom_one = 0;
    let mut castom_two = 0;
    let mut castom_tips = String::new();

    println!("{}", "
 _______  ______  __    _ 
|       ||    _ ||  |  | |
|    ___||   | |||   |_| |
|   | __ |   |_|||       |
|   ||  ||    __||  _    |
|   |_| ||   |\\\\ | | |   |
|_______||___| \\\\|_|  |__|
    ".purple().bold());
    println!("{}", "Добро пожаловать в GRN Definitive Edition!".purple().bold());  
    'start: loop {
        println!("{}", "1)Играть, 2)Описание".blue());

        io::stdin()
            .read_line(&mut setting_or_game)
            .expect("Error 16");
        
        if setting_or_game.trim() == "2"||setting_or_game.trim().to_lowercase() == "two"||setting_or_game.trim().to_lowercase() == "описание"{
            println!("
Суть данной игры — улучшить оригинальную 'Угадайку' [Guessing Game] из официальной обучающей
книги по Rust [The Rust Book]. В то время как оригинальный код занимает около 30 строк,
 версия Definitive Edition содержит 325+ строк");
            println!(" ");
            println!("
Также в игре присудствуют команды. Их можно ввести почти в любом месте,
чтобы начать ввод команды, используйте символ [/].
На данный момент доступны следующие команды: /stop, /cheats");
            println!(" ");
            println!("
Конечно же, в игре есть пасхалки. Активировать их можно с помощью команд через [/]. 
На данный момент доступны: /richard, /latuta.");
            println!("");
            println!("Нажмите любую клавишу для возвращения в главное меню");

            setting_or_game.clear();
            io::stdin()
                .read_line(&mut return_to_menu)
                .expect("Error 17");

            match return_to_menu.trim() {
                _ => continue
            }
            } else if setting_or_game.trim() == "1" || setting_or_game.trim().to_lowercase() == "one" || setting_or_game.trim().to_lowercase() == "играть"{
                    loop {
                println!("{}", "На какой сложности вы бы хотели сыграть?".blue()); 
                println!("{}", "1) Лёгкая [1-50], 2) Нормальная [1-100], 3) Сложная [1-1000],".green());     
                println!("{}", "4)Невозможная [1-10000] + нет подсказок больше/меньше, 5) Кастомная [самостоятельно указать диапазон]".green());

                io::stdin()
                    .read_line(&mut mode)
                    .expect("Error 5");

                match mode.trim() {
                    "1" => break,
                    "2" => break,
                    "3" => break,
                    "4" => break,
                    "5" => {castom = 1; break},
                    STOP_COMAND => break 'start,
                    CHEATS_COMAND => {
                        setting_or_game.clear();
                        mode.clear();
                        if cheats_on == 0 {
                            cheats_on = 1; println!("Читы успешно активированы"); continue;
                        }else if cheats_on == 1{
                            cheats_on = 0; println!("Читы успешно выключены"); continue; 
                        }else {
                            println!("Error 22")
                        }
                    }
                    LATUTA_EASTER => {setting_or_game.clear(); println!("Латута тварь"); continue;}
                    RICHARD_EASTER => {setting_or_game.clear(); println!("Ну он крутой"); continue;}
                    &_ => continue
                }
                
            }

            loop {
                
                if castom == 1 {

                    castom = 0;

                    loop {
                        println!("");
                        println!("Первое число конфигурации: ");

                        io::stdin()
                            .read_line(&mut castom_num_one)
                            .expect("Error 9");

                        castom_one = match castom_num_one.trim().parse() {
                            Ok(user_num) => user_num,
                            Err(_) => {println!("Error 10, подсказка: возможно, вы ввели данные типа str, попробуйте ввести только числа"); continue;}  
                        };
                        break;
                    };

                loop {
                    println!("");
                    println!("Второе число конфигурации: ");

                    io::stdin()
                        .read_line(&mut castom_num_two)
                        .expect("Error 11");

                    castom_two = match castom_num_two.trim().parse() {
                        Ok(user_num) => user_num,
                        Err(_) => {println!("Error 12, подсказка: возможно, вы ввели данные типа str, попробуйте ввести только числа"); continue;}                    
                    };

                    if castom_two < castom_one {
                        println!("Error 13, подсказка: большее число не должно быть меньше меньшего"); castom_num_two.clear(); continue; 
                    }
                    break;
                }
                
                loop {
                    println!("Выключить подсказки больше/меньше? [Да/Нет]");
                    io::stdin()
                        .read_line(&mut castom_tips)
                        .expect("Error 14");

                    match castom_tips.trim() {
                        "да"|"y" => {tips_off = 1; break},
                        "нет"|"n" => {tips_off = 0; break},
                        _ => {println!("Error 15, подсказка: попробуйте ввести [1] или [2]"); println!(""); castom_tips.clear(); continue}             
                        }
                    }   
                } 

                let random_number = match mode.trim() {
                    "1" => rand::thread_rng().gen_range(1..=50),
                    "2" => rand::thread_rng().gen_range(1..=100),
                    "3" => rand::thread_rng().gen_range(1..=1000),
                    "4" => {tips_off = 1; rand::thread_rng().gen_range(1..=10000)},
                    "5" => {rand::thread_rng().gen_range(castom_one..=castom_two)}
                    _  => todo!()
                };

            if cheats_on == 1 {println!("[ЧИТЫ]неизвестное число: {}", random_number);}

            'game: loop {
                round += 1;
                
                let mut user_continue = String::new();
                let mut user_guess = String::new(); 

                println!(" ");
                println!("Твоя догадка:");

                io::stdin()
                    .read_line(&mut user_guess)
                    .expect("Error 1");

                if user_guess.trim().to_lowercase() == STOP_COMAND{
                    break 'start;
                } else if user_guess.trim().to_lowercase() == CHEATS_COMAND {
                    user_guess.clear();
                    
                    if cheats_on == 0 {
                        cheats_on = 1; println!("Читы успешно активированы"); continue;
                    }else if cheats_on == 1{
                        cheats_on = 0; println!("Читы успешно выключены"); continue; 
                    }else {
                        println!("Error 21")
                    }
                } else if user_guess.trim().to_lowercase() == LATUTA_EASTER {
                    user_guess.clear();
                    println!("Латута тварь");
                    continue;
                } else if user_guess.trim().to_lowercase() == RICHARD_EASTER {
                    println!("Он крутой");
                    continue;
                }

                let user_guess: u32 = match user_guess.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {println!("Error 2, ошибка ввода, подсказка: ввод не поддерживает текстовый формат"); continue;} 
                };

                if mode.trim() == "1" && user_guess > EASY_RANDOM_MAX || mode.trim() == "1" && user_guess < RANDOM_MIN{
                    println!("{}","Эй, вообще-то мы договаривались играть в диапазоне [1-50], попытка не засчитана!".yellow());
                } else if mode.trim() == "2" && user_guess > NORMAL_RANDOM_MAX || mode.trim() == "2" && user_guess < RANDOM_MIN{
                    println!("{}","Эй, вообще-то мы договаривались играть в диапазоне [1-100], попытка не засчитана!".yellow());                
                } else if mode.trim() == "3" && user_guess > HARD_RANDOM_MAX || mode.trim() == "3" && user_guess < RANDOM_MIN{
                    println!("{}","Эй, вообще-то мы договаривались играть в диапазоне [1-1000], попытка не засчитана!".yellow());
                } else if mode.trim() == "4" && user_guess > UNPOSIBLE_RANDOM_MAX || mode.trim() == "4" && user_guess < RANDOM_MIN{
                    println!("{}","Эй, вообще-то мы договаривались играть в диапазоне [1-10000], попытка не засчитана!".yellow());
                }else if mode.trim() == "5" && user_guess > castom_two || mode.trim() == "5" && user_guess < castom_one{
                    print!("{}", "Ей, вообще-то мы договаривались играть в радиусе ".yellow());
                    print!("[{}-{}]", castom_one, castom_two);
                    println!("{}", ", попытка не засчитана!".yellow())
                }else{
                    best_result_time += 1;
                    
                    match user_guess.cmp(&random_number) {
                    Ordering::Greater => 
                        {if tips_off == 0 {println!("{}", "Число меньше".yellow()); trying += 1}
                        else if tips_off == 1 {println!("{}", "Неверно".yellow()); trying += 1}
                        else {println!("Error 7");}},
                    Ordering::Less => 
                        {if tips_off == 0 {println!("{}", "Число больше".yellow()); trying += 1;}
                        else if tips_off == 1 {println!("{}", "Неверно".yellow()); trying += 1}
                        else {println!("Error 8");}},
                    Ordering::Equal => {
                        trying += 1;                    
                        
                        if round%2 == 0 && trying<best_trying_one {
                            best_trying_one = trying
                        } else if round%2 != 0 && trying<best_trying_two {
                            best_trying_two = trying
                        }
                        
                        if trying<=0 {
                            println!("{}", "Ты выиграл! Каким образом!?".red().bold());
                        } else if trying==1 {
                            println!("{}", "Ты выиграл! Даже не знаю, как тебе это удалось!".purple().bold())
                        } else if trying>1 && trying<=5 {
                            println!("{}", "Ты выиграл! Ричард горд твоим мастерством!".purple().bold())
                        } else if trying>5 && trying <=15 {
                            println!("{}", "Ты выиграл! Молодец!".green().bold())
                        } else if trying==994 || trying==993 {
                            println!("{}", "Ты выиграл! Пасхалочка".purple().bold())
                        } else {
                            println!("{}", "Ты выиграл!".green().bold())                        
                        }

                        print!("{}", "Угадано спустя попыток: ".blue());
                        println!("{}", trying); 

                        if best_trying_one > best_trying_two {
                            best_trying_duo = best_trying_two
                        } else if best_trying_one < best_trying_two {
                            best_trying_duo = best_trying_one
                        }
                        
                        if best_result_time >= 2 {
                            print!("{}", "Твой лучший результат в этой сессии: ".yellow());
                            println!("{}", best_trying_duo);
                        }
                        

                        loop {
                        println!("");
                        println!("Продолжить? [Да/Нет]");
                    
                        io::stdin()
                            .read_line(&mut user_continue)
                            .expect("Error 3");
                                                        
                            if user_continue.trim().to_lowercase() == "да" || user_continue.trim().to_lowercase() == "y" {
                                trying = 0; break 'game;
                            } else if user_continue.trim().to_lowercase() == "нет" || user_continue.trim().to_lowercase() == "n" {
                                break 'start;
                            } else if user_continue.trim().to_lowercase() == STOP_COMAND {
                                break 'start;
                            } else if user_continue.trim().to_ascii_lowercase() == CHEATS_COMAND{
                                user_continue.clear();
                                if cheats_on == 0 {
                                    cheats_on = 1; println!("Читы успешно активированы"); continue;
                                }else if cheats_on == 1{
                                    cheats_on = 0; println!("Читы успешно выключены"); continue; 
                                }else {
                                    println!("Error 20")
                                }                          
                            } else if user_continue.trim().to_lowercase() == LATUTA_EASTER {
                                setting_or_game.clear();
                                println!("Латута тварь");
                                continue;                               
                            } else if user_continue.trim().to_lowercase() == RICHARD_EASTER  {
                                setting_or_game.clear();
                                println!("Он крутой");  
                                continue;                          
                            } else {
                                println!("Error 4, подсказка: попробуйте написать [Да] или [Нет] "); continue;                                   
                            }
                        }   
                    }
                }
            }
        }
    }
        } else if setting_or_game.trim().to_lowercase() == STOP_COMAND{
            break 'start;
        } else if setting_or_game.trim().to_lowercase() == CHEATS_COMAND{
            setting_or_game.clear();
            if cheats_on == 0 {
                cheats_on = 1; println!("Читы успешно активированы"); continue;
            }else if cheats_on == 1{
                cheats_on = 0; println!("Читы успешно выключены"); continue; 
            }else {
                println!("Error 19")
            }
        } else if setting_or_game.trim().to_lowercase() == LATUTA_EASTER {
            setting_or_game.clear();
            println!("Латута тварь");
            continue;
        } else if setting_or_game.trim().to_lowercase() == RICHARD_EASTER   {
            setting_or_game.clear();
            println!("Он крутой");
            continue;
        } else {
            println!("Error 18, подсказка: программе не удалось использовать ваш ввод, попробуйте ещё раз"); setting_or_game.clear(); continue;        
        }
    } 
}
