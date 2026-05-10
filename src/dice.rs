use std::collections::BTreeMap;

use rand::Rng;

pub struct Dice {
    number: u32,
    sides: u32,
    modifier: i32,
}

impl Dice {
    #[must_use]
    pub fn new(number: u32, sides: u32, modifier: i32) -> Dice {
        assert!(sides > 0, "sides must be > 0");
        Self {
            number,
            sides,
            modifier,
        }
    }

    pub fn roll(&self) -> i32 {
        let mut random = rand::rng();
        random.random_range(1..=self.sides) as i32 + self.modifier
    }

    pub fn roll_multiple(&self) -> i32 {
        let mut sum: i32 = 0;
        for _i in 0..self.number {
            let result = self.roll();
            sum += result;
            println!("{}", result);
        }
        if self.modifier != 0 {
            println!("Total: {}", sum);
        }
        sum
    }

    pub fn test(&self) {
        let mut counts: BTreeMap<i32, i32> = BTreeMap::new();

        for _ in 0..self.number {
            let result = self.roll();
            *counts.entry(result).or_insert(0) += 1;
        }

        println!("-Roll Count-");
        for (face, number) in &counts {
            println!("{}: {}", face, number);
        }

        let mut chi_sum = 0.0;
        let expected = self.number as f64 / self.sides as f64;
        for (_, number) in counts {
            let chi = ((number as f64 - expected).powi(2)) / expected;
            chi_sum += chi;
        }
        println!("Chi Squared: {}", chi_sum);
    }
}
