use std::thread::sleep;
use std::time::Duration;

/**
* The match statement in Rust is equal to the switch-case statement in other languages.
*/
fn main() {
    // 1
    {
        let var: i32 = 5;
        
        match var { 
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            4 => println!("four"),
            5 => println!("five"),
            6 => println!("six"),
            7 => println!("seven"),
            _ => println!("something else idk"),
        }
    }
    
    // 2
    {
        let action: &str = "play";
        
        match action {
            "play" => {
                println!("Playing...");
                sleep(Duration::from_secs(1));
                print!("Started playing successfully!");
            }
            "pause" => {
                print!("Pausing...");
                sleep(Duration::from_secs(1));
                print!("Pause successfully!");
            }
            _ => {
                println!("Unknown action.");
            }
        }
    }
    
    // 3
    {
        let var: i32 = 5;
        
        match var {
            1 => println!("var is one"),
            2..=5 => println!("var is in interval [2; 5]"),
            6 | 7 => println!("var is either six or seven"),
            8 | 10..100 => println!("var is either eight or in interval [10; 100)"),
            _ => println!("var is something else"),
        }
    }
    
    // 4
    {
        
    }
}