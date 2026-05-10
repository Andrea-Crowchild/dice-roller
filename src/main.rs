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
    let cli = Cli::parse();

    let dice = Dice::new(cli.number, cli.sides, cli.modifier.unwrap_or(0));

    if cli.test {
        dice.test();
    } else {
        dice.roll_multiple();
    }
}
