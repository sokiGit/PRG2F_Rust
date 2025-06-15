use std::ops::Add;

fn main() {
    let a: &str = "String literal";             // Aka this is a string slice initialized with a string literal
    
    let b: String = "Hello".to_string();        // Can use the ToString trait to turn &str reference into owned String
    let c: String = "World".to_owned();         // Also can use the ToOwned trait
    let d: String = String::from("Rust");    // The preferred way to create String from &str
    
    let e: String = b.to_owned() + " " + &c;               // Have to add string literals to String, b would be moved here, so we have to 'copy' it via the ToOwned trait
    let f: String = b.add(", Meow!");
    
    // String methods
    println!("{}", a.trim());
    println!("{}", a.len());
    println!("{}", a.find("ing").expect("Couldn't find \"ing\"."));
    println!("{}", a.replace("lit", "meow"));
    println!("{}", a.to_lowercase());
    println!("{}", a.to_uppercase());
    // etc...
    
    // Substrings
    println!("{}", &"Hello, World!"[..5]);      // Gets the first five characters ("Hello")
    println!("{}", &"Hello, World!"[7..12]);    // Gets characters from 7th to 12th place ("World")
    
    let a: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed...";
    
    println!("{}", &a[5..a.len()]);              // Gets characters from the fifth to the end
    println!("{}", &a[a.len() - 5..a.len()]);    // Gets last 5 characters
    
    // Comparing strings
    dbg!("A" == &String::from("A"));                    // true
    dbg!(&"B" == &String::from("B"));                   // true
    dbg!("C" == "C");                                      // true
    dbg!(String::from("D") == String::from("D"));    // true
    dbg!(String::from("E") == "E".to_string());         // true
    
    // Cannot compare   str     to  String
    // Cannot compare   &str    to  &str
    
    // Can compare      str     to  &String
    // Can compare      &str    to  &String
    // Can compare      String  to  String
    
    dbg!("A".eq_ignore_ascii_case("A"));                    // Ignore case when comparing
    dbg!("B".eq_ignore_ascii_case(&"B"));                   // Also possible
    dbg!(String::from("C").eq_ignore_ascii_case("C"));   // This as well
    dbg!("D".eq_ignore_ascii_case(&String::from("D")));        // Yup, this too
    
    // Lexicographical Comparison
    dbg!("E" < "A");
    dbg!("F".cmp("G"));     // Cmp trait - orders the string's byte values, not necessarily the same as alphabetical order
}