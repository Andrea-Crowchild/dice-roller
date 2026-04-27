
use rand::Rng;

pub struct Dice {
    number: u32,
    sides: u32,
    modifier: i32,
}
// test mode and verbose mode are not yet implemented and new() will
// be designed to set a default value of false in both cases.
pub struct Config {
    pub number: u32,
    pub sides: u32,
    pub modifier: i32,
    pub test_mode: bool,
    pub verbose: bool,
}



impl Dice {
    pub fn new(number: u32, sides: u32, modifier: i32) -> Dice {
        Dice {
            number,
            sides,
            modifier,
        }
    }

    pub fn roll(&self){
        let mut sum: i32 = 0; 
        let mut random = rand::thread_rng();
        for _i in 0..self.number{
            let result = random.gen_range(1..=self.sides);
            sum += result as i32;
            println!("{}", result);
        }
        if self.modifier != 0{
            sum += self.modifier;
            println!("Total: {}", sum);
        }
    }
}


