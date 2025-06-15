use std::io::stdin;
use rand::{Rng, SeedableRng};

fn main() {
    // 1
    {
        let num: i32 = 123;

        println!("{} is {}divisible by 5", num, if num % 5 == 0 { "" } else { "not " });
    }

    // 2
    {
        let mut rng = rand::rng();
        
        let mut n1: i32 = rng.random_range(1..=6);
        if n1 == 0 { n1 = 1; }
        
        let mut n2: i32 = rng.random_range(1..=6);
        if n2 == 0 { n2 = 1; }
        
        println!("Numbers: {}, {} - You {}!", n1, n2, if n1 + n2 > 10 { "won" } else { "lost" })
    }
    
    // 3
    {
        println!("Enter a number that's in this set of real numbers: [-100; 100] \\ 0");
        let mut user_input: String = String::new();
        stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        
        let num: i32 = user_input
            .trim()
            .parse::<i32>()
            .expect("Failed to parse integer");
        
        println!("{} is {}in set: [-100; 100] \\ 0 ", num, if num != 0 && (-100..=100).contains(&num) { "" } else { "not " });
    }

    // 4
    {
        println!("Enter your age:");
        let mut user_input: String = String::new();
        stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        
        let age: u32 = user_input
            .trim()
            .parse()
            .expect("Failed to parse unsigned integer");

        const ORIGINAL_PRICE: f64 = 0.0;
        let mut discount: f64 = 0.0;
        
        if (1 <= age && age <= 12) || (age >= 70) {
            discount = 0.40;
        } else if age >= 13 && age < 18 { 
            discount = 0.2;
        }
        
        println!("Your final price is {} (discount: {}%)", ORIGINAL_PRICE * discount, discount * 100.0);
    }
    
    // 5
    {
        const PRICE_PER_M2: f64 = 640.0;
        
        println!("Enter parcel width:");
        let width: f64 = {
            let mut input = String::new();
            stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            input
                .trim()
                .parse()
                .expect("Failed to parse unsigned integer")
        };
        
        println!("Enter parcel height:");
        let height: f64 = {
            let mut input = String::new();
            stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            input
                .trim()
                .parse()
                .expect("Failed to parse unsigned integer")
        };
        
        println!("Enter your budget:");
        let budget: f64 = {
            let mut input = String::new();
            stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            input
                .trim()
                .parse()
                .expect("Failed to parse unsigned integer")
        };
        
        let final_price = PRICE_PER_M2 * width * height;
        
        println!("Your final price is {}$. That {} into your budget.", final_price, if (final_price > budget) { "doesn't fit" } else { "fits" });
    }
}