#![allow(unused)]

use dice::{Config, Dice};
use std::env;

mod dice;

fn main() {
    let config = match parse_args() {
        Ok(n) => n,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let dice = Dice::new(
        config.number,
        config.sides,
        config.modifier,
        config.test_mode,
    );
    if dice.test_mode {
        dice.test();
    } else {
        dice.roll();
    }
}

fn parse_args() -> Result<Config, String> {
    let args: Vec<String> = env::args().collect();
    let modifier;
    let test_mode;

    // this makes sure there are at least two arguments
    if args.len() < 2 {
        return Err(String::from("Invalid arguments, try dice -h for help!"));
    }
    if args[1] == "-h" {
        return Err(String::from(
            "Syntax is \"dice <no> <sides> <modifier>\" \
            where no is the number of dice and sides is the number of sides. \
            Modifier is optional. \nUse \"dice <no> <sides> --test to \
            engage test mode and calculate chi squared.",
        ));
    }
    if args.len() < 3 {
        return Err(String::from("Invalid arguments, try dice -h for help!"));
    }
    // it's necessary to make sure args[3] exists prior to testing
    // it for test mode
    if args.len() > 3 {
        modifier =  args[3].parse().unwrap_or(0);
        test_mode = args[3] == "--test";
    } else {
        modifier = 0;
        test_mode = false;
    }

    let number = match args[1].parse() {
        Ok(n) => n,
        Err(_) => return Err(String::from("Invalid number of dice!")),
    };

    let sides = match args[2].parse() {
        Ok(n) => n,
        Err(_) => return Err(String::from("Invalid number of sides!")),
    };

    Ok(Config {
        number,
        sides,
        modifier,
        test_mode,
    })
}
