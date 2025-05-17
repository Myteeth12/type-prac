use colored::Colorize;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};
use std::io::{self, Read, stdout};

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
    while running_ {
        execute!(stdout(), MoveTo(0, 0), Clear(ClearType::All)).unwrap();
        println!(
            "{}{}{}",
            "--INFINITE MODE------".blue(),
            "[ctrl+c to exit or press 'x']".red(),
            "-->".blue()
        );
        let test1 = "Testword";
        let test2: Vec<&str> = test1.split("").filter(|i| !i.is_empty()).collect();
        println!("\n  {}", "LMAO".yellow());
        println!("{:#?}", test2);

        for key in io::stdin().bytes() {
            let keybyte = key.unwrap();
            let keychar = keybyte as char;

            if keybyte == 003 as u8 {
                running_ = false;
                break;
            }

            execute!(stdout(), MoveTo(0, 0), Clear(ClearType::All)).unwrap();
            break;
        }
    }
}
