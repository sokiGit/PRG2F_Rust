fn main() {
    // Multiples of 5 from 1 to 1000
    for i in (5..=1000).step_by(5) {
        println!("{}", i);
    }
    
    // All numbers from 100 to 0
    for i in (0..=100).rev() {
        println!("{}", i);
    }
    
    // Second power of numbers from 100 to 0
    for i in (0..=100).rev() {
        println!("{}", i * i);
    }
}