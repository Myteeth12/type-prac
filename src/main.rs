use colored::Colorize;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};
use std::io::{self, Read, stdout};

mod scripts;

fn main() {
    let mut main_run = true;
    while main_run {
        execute!(stdout(), MoveTo(0, 0), Clear(ClearType::All)).unwrap();
        println!(
            "{}{}{}",
            "--TYPE PRACTICE------".blue(),
            "[ctrl+c to exit or press 'x']".red(),
            "-->".blue()
        );
        println!("\n  {}", "   [type '1'] ----- Infinite Mode ]".yellow());
        println!("  {}", "   [type '2'] ----- Random Mode ]".yellow());
        println!("  {}", "   [type '3'] ----- Custom ]".yellow());
        println!("testthing:{}", scripts::word_random());

        enable_raw_mode().unwrap();
        for key in io::stdin().bytes() {
            let keybyte = key.unwrap();
            let keychar = keybyte as char;

            if keybyte == 003 as u8 {
                disable_raw_mode().unwrap();
                main_run = false;
                break;
            } else if keychar == 'x' {
                disable_raw_mode().unwrap();
                main_run = false;
                break;
            } else if keychar == '1' {
                infinite_mode();
            }

            break;
            //println!("character: {}", keychar);
        }
    }

    println!("{}", "ENDING BYE!  ".yellow());
}

fn infinite_mode() {
    let mut running_ = true;
    let words_gen = [
        scripts::word_random(),
        scripts::word_random(),
        scripts::word_random(),
    ];
    let words_quest = words_gen.join(" ");
    let mut words_all: Vec<&str> = words_quest.split("").collect();
    words_all.remove(0);
    words_all.remove(words_all.len() - 1);
    let mut user_typed: Vec<String> = Vec::new();

    while running_ {
        execute!(stdout(), MoveTo(0, 0), Clear(ClearType::All)).unwrap();
        println!(
            "{}{}{}",
            "--INFINITE MODE------".blue(),
            "[ctrl+c to exit]".red(),
            "-->".blue()
        );
        if words_quest.len() == user_typed.join("").len() {
            break;
        }
        println!("\n");
        for (i, lt) in words_all.iter().enumerate() {
            if i < user_typed.len() && lt.to_string() == user_typed[i].as_str() {
                print!("{}", lt.green());
            } else if i < user_typed.len() && lt.to_string() != user_typed[i].as_str() {
                print!("{}", lt.red());
            } else if i == user_typed.len() {
                print!("{}", lt.black().on_white());
            } else {
                print!("{}", lt.blue());
            }
        }
        //let test1 = "Testword";
        //let test2: Vec<&str> = test1.split("").filter(|i| !i.is_empty()).collect();
        //println!("\n {}", words_quest.blue());
        println!("\n  {}", "LMAO".yellow());

        for key in io::stdin().bytes() {
            let keybyte = key.unwrap();
            let keychar = keybyte as char;

            if keybyte == 003 as u8 {
                running_ = false;
                break;
            } else {
                user_typed.push(keychar.to_string());
            }

            execute!(stdout(), MoveTo(0, 0), Clear(ClearType::All)).unwrap();
            break;
        }
    }
}
