use std::env;
use rand::Rng;



fn main() {
    // initialize sum variable to 0 
    let mut sum: i32 = 0;
    //take command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Invalid input. Use -h for help.");
        return;
    }
    if args[1] == "-h" {
        println!("Syntax is dice <no> <sides>");
        return;
    }

    let no_of_dice: u32 = match args[1].parse(){
        Ok(n)  => n,
        Err(_) => {
            println!("Invalid input. Use -h for help.");
            return;
        }
    };
    let sides: u32 = match args[2].parse(){
        Ok(n) => n,
        Err(_) => {
            println!("Invalid input. Use -h for help.");
            return;
        }
    };
    // rolling logic
    for _i in 0..no_of_dice{
        let result = rand::thread_rng().gen_range(1..=sides);
        sum += result as i32;
        println!("{}", result);
    }
    println!("Total: {}", sum);

}
