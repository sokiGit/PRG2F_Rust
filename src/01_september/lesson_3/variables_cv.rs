use std::io::{stdin, Read};

fn main() {
    let age: i32 = 0;
    let pi: f64 = std::f64::consts::PI;
    let is_true: bool = true;
    let big_a: char = 'A';
    let name: &str = "Joe";
    let owned_name: String = String::from("Mama");      // Can also use "Mama".to_owned() or "Mama".to_string()

    /*
        CV 1
        Save any two numbers into two variables.
        After that:
        Print them, swap them, print them again
    */
    let mut a = 10;
    let mut b = 20;

    println!("a: {}, b: {}", a, b);

    let tmp = a;
    a = b;
    b = tmp;

    println!("a: {}, b: {}", a, b);

    /*
        CV 2
        Write a basic calculator which can do +, -, *, / on two numbers from the set of real numbers.
        Print out the result
    */

    let mut a_str: String = String::new();
    stdin()
        .read_line(&mut a_str)
        .expect("Failed to read line");

    let a: f64 = a_str
        .trim()
        .parse()
        .expect("Please type a number!");

    let mut b_str: String = String::new();
    stdin()
        .read_line(&mut b_str)
        .expect("Failed to read line");

    let b: f64 = b_str
        .trim()
        .parse()
        .expect("Please type a number!");

    println!("{} + {} = {}", a, b, a + b);
    println!("{} - {} = {}", a, b, a - b);
    println!("{} * {} = {}", a, b, a * b);
    println!("{} / {} = {}", a, b, a / b);

    /*
        CV 3
        Save whether the sum of any three lowercase characters is larger than 300 into a boolean variable.
        Print out the result.
     */

    let is_big: bool = 'f' as u32 + 'a' as u32 + 'a' as u32 > 300;

    println!("The sum of the u32 representation of these chars is {}.", if is_big { "big" } else { "small" });
    
    /*
        CV 4
        Ask the user for their forename, surname, postal code and birth number.
        Print out this information.
     */
    
    let mut forename: String = String::new();
    let mut surname: String = String::new();
    let mut postal_code: String = String::new();
    let mut birth_number: String = String::new();

    println!("Enter your forename: ");
    stdin()
        .read_line(&mut forename)
        .expect("Failed to read line.");
    
    println!("Enter your surname: ");
    stdin()
        .read_line(&mut surname)
        .expect("Failed to read line.");
    
    println!("Enter your postal code: ");
    stdin()
        .read_line(&mut postal_code)
        .expect("Failed to read line.");
    
    println!("Enter birth number: ");
    stdin()
        .read_line(&mut birth_number)
        .expect("Failed to read line.");
    
    println!("Forename: {}\
    Surname: {}\
    Postal Code: {}\
    Birth Number: {}\
    ", forename, surname, postal_code, birth_number);

    /*
        BONUS
        CV 5
        Generate a chessboard of size N×N, where N is a user input.
     */
    let mut chessboard_size_str: String = String::new();
    stdin()
        .read_line(&mut chessboard_size_str)
        .expect("Failed to read line.");
    
    let chessboard_size = chessboard_size_str
        .trim()
        .parse()
        .expect("Please type a number!");

    for y in 0..chessboard_size {
        for x in 0..chessboard_size {
            if (y + x) % 2 == 0 { print!("■ "); } 
            else { print!("□ "); }
        }
        println!();
    }
}
