use clap::Parser;
mod dice;
use dice::Dice;

#[derive(Parser)]
struct Cli {
    number: u32,
    sides: u32,
    #[arg(short, long, conflicts_with = "modifier")]
    test: bool,

    modifier: Option<i32>,
}

fn main() {
    let config = Cli::parse();

    let dice = Dice::new(config.number, config.sides, config.modifier.unwrap_or(0));

    if config.test {
        dice.test();
    } else {
        dice.roll_multiple();
    }
}
