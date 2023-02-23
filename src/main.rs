extern crate chrono;
extern crate timer;

use std::sync::mpsc::channel;

fn main() {
    loop {
        print_menu();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input: u32 = input.trim().parse().unwrap();

        match input {
            1 => {
                println!("Pomodoro");
                delay(25 * 60);
            },
            2 => {
                println!("Short Break");
                delay(5 * 60)
            },
            3 => {
                println!("Long Break");
                delay(15 * 60)
            },
            4 => {
                println!("Exit");
                break;
            },
            _ => {
                println!("Invalid input");
            }
        }
    }
}

fn print_menu() {
    println!("1. Pomodoro");
    println!("2. Short Break");
    println!("3. Long Break");
    println!("4. Exit");
}

fn delay(d: i64) {
    let timer = timer::Timer::new();
    let (tx, rx) = channel();

    let _guard = timer.schedule_with_delay(chrono::Duration::seconds(d), move || {
        let _ = tx.send(());
    });

    rx.recv().unwrap();

    println!("Time's up!");
}