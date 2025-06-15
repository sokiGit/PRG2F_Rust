use std::io::stdin;
use regex::Regex;

fn main() {
    {
        // Assignment 1
        println!("Enter a number:");
        let mut user_input = String::new();
        stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        
        let num: i32 = user_input
            .trim()
            .parse()
            .expect("Please type a number!");
        
        dbg!(num);
        dbg!(num + 10);
    }

    {
        // Assignment 2
        let mut user_input = String::new();
        stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        
        user_input = user_input.trim().to_lowercase();

        println!("Starts with \"Rust\": {}", user_input.starts_with("rust "));
        println!("Starts with \"rocks\": {}", user_input.ends_with(" rocks"));
    }

    {
        // Assignment 3
        fn get_usize_from_user() -> usize {
            let mut user_input: String = String::new();
            
            stdin()
                .read_line(&mut user_input)
                .expect("Failed to read line");
            
            user_input
                .trim()
                .parse()
                .expect("Please type a number!")
        }

        println!("Enter a string: ");
        let mut user_input: String = String::new();
        stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        
        println!("Enter a starting index: ");
        let start_index: usize = get_usize_from_user();
        
        println!("Enter an starting index: ");
        let end_index: usize = get_usize_from_user();
        
        dbg!(&user_input[start_index..end_index]);
    }

    {
        // Advanced Assignment 1
        let regex = Regex::new("\\d+").unwrap();
        
        let mut user_input = String::new();
        stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        dbg!(regex.find_iter(&user_input)
            .filter_map(|digits| digits.as_str().parse::<i32>().ok())
            .collect::<Vec<i32>>());
    }

    {
        // Advanced Assignment 2
        // ERR: Look-around, including look-ahead and look-behind, is not supported
        /*
        
        let regex = Regex::new(r"(?<=\w)+\.\w+(?=\s|,|$)").unwrap();
        
        println!("Enter a comma-separated list of files: ");
        let mut user_input = String::new();
        stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        
        dbg!(regex.find_iter(&user_input)
            .filter_map(|exts| Option::from(exts.as_str()))
            .collect::<Vec<&str>>());
            
         */
    }
}