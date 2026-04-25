#![allow(unused)]

use std::env;
use dice::{Dice, Config};

mod dice;


fn main() {
    let config = match parse_args() {
        Ok(n) => n,
        Err(e) => {
            println!("{}", e);
            return;
        },
    };
    let dice = Dice::new(config.number, config.sides);
    dice.roll();

}

fn parse_args() -> Result<Config, String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(String::from("Invalid arguments, try dice -h for help!"));
    }
    if args[1] == "-h" {
        return Err(String::from("Syntax is dice <no> <sides> where no is the number of dice and sides is the number of sides."));
    }
    if args.len() < 3 {
        return Err(String::from("Invalid arguments, try dice -h for help!"));
    }

    let number = match args[1].parse() {
        Ok(n) => n,
        Err(_) => return Err(String::from("Invalid number of dice!")),
    };

    let sides = match args[2].parse() {
        Ok(n) => n,
        Err(_) => return Err(String::from("Invalid number of sides!")),
    };

    Ok(Config{
        number,
        sides,
        test_mode: false,
        verbose: false,
    })
}