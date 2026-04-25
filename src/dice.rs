
use rand::Rng;

pub struct Dice {
    number: u32,
    sides: u32,
}
// test mode and verbose mode are not yet implemented and new() will
// be designed to set a default value of false in both cases.
pub struct Config {
    pub number: u32,
    pub sides: u32,
    pub test_mode: bool,
    pub verbose: bool,
}



impl Dice {
    pub fn new(number: u32, sides: u32) -> Dice {
        Dice {
            number,
            sides,
        }
    }

    pub fn roll(&self){
        let mut sum: u32 = 0; 
        for _i in 0..self.number{
            let result = rand::thread_rng().gen_range(1..=self.sides);
            sum += result;
            println!("{}", result);
        }
        println!("Total: {}", sum);
    }
}


