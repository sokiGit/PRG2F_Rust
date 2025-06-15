use std::io::{stdin, Read};

fn main() {
    {
        // Assignment 1: Sum of numbers
        /*
            Read numbers from user until they input 0.
            Sum up the inputted numbers.
         */

        fn get_f64_from_user() -> f64 {
            println!("Enter a number:");

            let mut user_input = String::new();
            stdin()
                .read_line(&mut user_input)
                .expect("Failed to read line");

            user_input
                .trim()
                .parse()
                .expect("Failed to parse string")
        }

        let mut numbers_list: Vec<f64> = vec!();
        loop {
            let input: f64 = get_f64_from_user();

            if input == 0.0 { break; }

            numbers_list.push(input);
        }

        println!("Sum of all numbers: {:?}", numbers_list.iter().sum::<f64>());
    }

    {
        // Assignment 2
        /*
            Read password from user.
            Compare to stored password.
            Grant access if correct within 3 attempts.
         */

        const CORRECT_PASSWORD: &str = "supersecretpassword";

        let mut attempts = 0;
        let has_access: bool = loop {
            attempts += 1;

            println!("Enter a password:");

            let mut user_input = String::new();
            stdin()
                .read_line(&mut user_input)
                .expect("Failed to read line");

            if user_input.trim().eq(CORRECT_PASSWORD) { break true; }
            else if attempts >= 3 { break false; }
        };
        if has_access {  println!("Access granted!"); }
        else { println!("Access denied!"); }
    }

    {
        // Assignment 3
        /*
            Get positive numbers from user.
            When user inputs a negative number, the program says how many even numbers they inputted and exits. 
         */
        
        let mut even_numbers: u32 = 0;
        
        loop {
            println!("Enter an integer (entering a negative number stops the loop):");
            let mut user_input: String = String::new();
            stdin()
                .read_line(&mut user_input)
                .expect("Failed to read line");
            
            let num: i32 = user_input
                .trim()
                .parse()
                .expect("Failed to parse string");

            if num < 0 { break; }
            else if num % 2 == 0 { even_numbers += 1; }
        }

        println!("Even numbers: {}", even_numbers);
    }

    {
        // Assignment 4
        let mut max: i32 = 0;

        loop {
            println!("Enter a number (entering 0 breaks the loop):");
            let mut user_input = String::new();
            stdin()
                .read_line(&mut user_input)
                .expect("Failed to read line");
            
            let num: i32 = user_input
                .trim()
                .parse()
                .expect("Failed to parse string");
            
            if num == 0 { break; }
            if num > max { max = num; }
        }
        
        println!("Max number: {}", max);
    }
}