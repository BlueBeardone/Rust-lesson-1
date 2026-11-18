use std::fmt;
use std::io::{self, Write};

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
enum MainPageButtons { 
    Hello = 1,
    Exit,
}

impl MainPageButtons {
    const ALL: [MainPageButtons; 2] = [
        MainPageButtons::Hello,
        MainPageButtons::Exit,
    ];
}

impl fmt::Display for MainPageButtons {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn main() {
    let mut running = true;

    while running == true{
        clearscreen::clear().expect("Failed to clear screen");
        io::stdout().flush().expect("Failed to flush stdout");

        println!("Type the number you want to do\n");

        for button in MainPageButtons::ALL {
            println!("{}. {}",button as u8, button);
        }

        println!("");
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<i32>().unwrap_or(0) {
            1 => hello(),
            2 => running = false,
            _ => println!("Must be 1 or 2"),
        }

    } 
} 

fn hello() {
    clearscreen::clear().expect("Failed to clear screen");
    let mut input = String::new();
    println!("Hello World");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("Enter your name");

    io::stdin().read_line(&mut input).expect("Failed to read line");

    println!("Hello {}", input.trim());

    io::stdin().read_line(&mut input).expect("Failed to read line");
}
