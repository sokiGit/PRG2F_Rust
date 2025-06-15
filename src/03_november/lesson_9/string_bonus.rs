use std::io::stdin;
use regex::Regex;

fn main() {
    {
        // Password validation
        let mut password: String = String::new();
        stdin()
            .read_line(&mut password)
            .expect("Did not enter a valid string");

        password = password.to_lowercase();

        if password.len() < 12
            || password.chars().next().unwrap().is_whitespace()
            || password.chars().last().unwrap().is_whitespace()
            || password.contains("red")
            || !password.contains("vacek")
            || !password.chars().last().expect("No last char.").eq(&'*')
        {
            println!("Not a valid password!");
        } else {
            println!("Password is valid!");
        }
    }

    {
        // Base conversion
        let mut user_input: String = String::new();
        stdin()
            .read_line(&mut user_input)
            .expect("Did not enter a valid string");

        let mut num: u32 = 0;

        for (i, c) in user_input.trim().chars().enumerate() {
            if c.eq(&'1') {
                num += 2_u32.pow(i as u32);
            } else if !c.eq(&'0') {
                break;
            }
        }

        dbg!(num);
    }

    {
        // Word count
        let mut user_input: String = String::new();
        stdin()
            .read_line(&mut user_input)
            .expect("Did not enter a valid string");

        let regex = Regex::new(r"\s+").unwrap();
        user_input = String::from(regex.replace_all(user_input.trim(), " "));

        dbg!(&user_input);
        dbg!(user_input.split(" ").count());
    }

    {
        // Palindrome
        let mut user_input: String = String::new();
        stdin()
            .read_line(&mut user_input)
            .expect("Did not enter a valid string");

        user_input = user_input.trim().to_lowercase();

        if user_input.chars().rev().collect::<String>().eq(&user_input) {
            println!("That is a palindrome.");
        } else {
            println!("That is not a palindrome.");
        }
    }

    {
        // Valid e-mail string
        let mut user_input: String = String::new();
        stdin()
            .read_line(&mut user_input)
            .expect("Did not enter a valid string");

        user_input = user_input.trim().to_string();

        let mail_regex = Regex::new(r"^[a-zA-Z][\w\-\\.]*@[\w\-\\.]+.[a-zA-Z]{2,}$").unwrap();    // Good enough...
        
        if mail_regex.is_match(&user_input) {
            println!("That is a valid email.");
        } else { 
            println!("That is not a valid email.");
        }
    }

    {
        // Abbreviation
        let mut user_input: String = String::new();
        stdin()
            .read_line(&mut user_input)
            .expect("Did not enter a valid string");
        
        user_input = user_input.trim().to_string();
        
        let mut abbr: String = String::new();
        
        for part in user_input.split(" ") {
            let c = part.chars().next().unwrap(); 
            if c.is_alphabetic() {
                abbr += &c.to_uppercase().to_string();
            }
        }
        
        dbg!(abbr);
    }
}