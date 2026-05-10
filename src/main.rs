use clap::Parser;
use dice::Dice;

mod dice;

#[derive(Parser)]
struct Cli {
    number: u32,
    sides: u32,
    #[arg(short, long)]
    test: bool,

    modifier: Option<i32>,
}

fn main() {
    let config = Cli::parse();
    if config.test && config.modifier.is_some() {
        eprintln!("Cannot use modifier and --test together");
        std::process::exit(1);
    }
    let dice = Dice::new(
        config.number,
        config.sides,
        config.modifier.unwrap_or(0),
        config.test,
    );

    if dice.get_test_mode() {
        dice.test();
    } else {
        dice.roll();
    }
}
