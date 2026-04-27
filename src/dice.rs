use std::collections::HashMap;

use rand::Rng;

pub struct Dice {
    number: u32,
    sides: u32,
    modifier: i32,
    pub test_mode: bool,
}
// test mode and verbose mode are not yet implemented and new() will
// be designed to set a default value of false in both cases.
pub struct Config {
    pub number: u32,
    pub sides: u32,
    pub modifier: i32,
    pub test_mode: bool,
}

impl Dice {
    pub fn new(number: u32, sides: u32, modifier: i32, test_mode: bool) -> Dice {
        Dice {
            number,
            sides,
            modifier,
            test_mode,
        }
    }

    pub fn roll(&self) {
        let mut sum: i32 = 0;
        let mut random = rand::thread_rng();
        for _i in 0..self.number {
            let result = random.gen_range(1..=self.sides);
            sum += result as i32;
            println!("{}", result);
        }
        if self.modifier != 0 {
            sum += self.modifier;
            println!("Total: {}", sum);
        }
        if self.test_mode {
            println!("Test mode enabled.");
        }
    }
    pub fn test(&self) {
        let mut dice_hash: HashMap<u32, u32> = HashMap::new();

        let mut random = rand::thread_rng();

        for _i in 0..self.number {
            let result = random.gen_range(1..=self.sides);
            let mut count = dice_hash.entry(result).or_insert(0);
            *count += 1;
        }

        let mut output: Vec<(u32, u32)> = dice_hash.into_iter().collect();
        output.sort_by_key(|(key, value)| *key);
        println!("-Roll Count-");
        for (face, number) in &output {
            println!("{}: {}", face, number);
        }

        let mut chi_sum = 0.0;
        for (face, number) in output {
            let expected = self.number as f64 / self.sides as f64;
            //  println!("{:?}", expected);
            let chi = ((number as f64 - expected).powi(2)) / expected;
            chi_sum += chi;
        }
        println!("Chi Squared: {}", chi_sum);
    }
}
