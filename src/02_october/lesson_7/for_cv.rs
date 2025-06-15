use std::io::stdin;
use rand::{rng, Rng};

fn main() {
    {
        // Assignment 1
        fn get_i32_from_user() -> i32 {
            let mut user_input: String = String::new();
            stdin().read_line(&mut user_input)
                .expect("Failed to read line");

            user_input
                .trim()
                .parse::<i32>()
                .expect("Please type a number!")
        }

        fn f(x: f64) -> f64 { 2.0 * x - 3.0 }

        println!("Enter lower bound:");
        let min = get_i32_from_user();

        println!("Enter upper bound:");
        let max = get_i32_from_user();

        for i in min..=max { println!("f({}) = {}", i, f(i as f64)); }
    }

    {
        // Assignment 2
        println!("Enter a positive integer:");

        let mut user_input: String = String::new();
        stdin().read_line(&mut user_input)
            .expect("Failed to read line");

        let num: u32 = user_input
            .trim()
            .parse::<u32>()
            .expect("Please type a number!");

        print!("sum = ");
        for i in 1..=num {
            print!("{}{}", if i != 1 { " + " } else {""}, i);
        }
        println!(" = {}", (1..=num).sum::<u32>());
    }

    {
        // Assignment 3
        println!("Enter a positive integer:");

        let mut user_input: String = String::new();
        stdin().read_line(&mut user_input)
            .expect("Failed to read line");

        let num: u32 = user_input
            .trim()
            .parse::<u32>()
            .expect("Please type a number!");

        print!("product = ");
        for i in (1..=num).rev() {
            print!("{}{}", if i != num { " + " } else {""}, i);
        }
        println!(" = {}", (1..=num).product::<u32>());
    }

    {
        // Assignment 4
        let mut tails_total: u32 = 0;
        let mut heads_total: u32 = 0;

        for _ in 0..10_000_000 {
            if rng().random_bool(0.5) {
                tails_total += 1;
            } else {
                heads_total += 1;
            }
        }

        println!("Tails probability: {}%", tails_total as f64 / 10_000_000.0 * 100.0);
        println!("Heads probability: {}%", heads_total as f64 / 10_000_000.0 * 100.0);
    }

    {
        // Assignment 5
        let mut threes: u32 = 0;

        for _ in 0..10_000_000 {
            if rng().random_range(1..=6) == 3 { threes += 1; }
        }

        println!("Three probability: {}%", threes as f64 / 10_000_000.0 * 100.0);
    }

    {
        // Assignment 6
        let mut user_input: String = String::new();
        stdin().read_line(&mut user_input)
            .expect("Failed to read line");

        let height: u32 = user_input
            .trim()
            .parse()
            .expect("Please type a number!");

        for y in 0..height {
            for _ in 0..=y {
                print!("*");
            }
            println!();
        }

        println!();

        for y in (0..height).rev() {
            for _ in 0..=y {
                print!("*");
            }
            println!();
        }

        println!();

        for y in 0..height {
            for x in 0..height {
                print!("{}", if x >= y { "*" } else { " " });
            }
            println!()
        }

        println!();

        for y in (0..height).rev() {
            for x in 0..height {
                print!("{}", if x >= y { "*" } else { " " });
            }
            println!()
        }

        println!();

        for y in 0..height {
            for x in 0..height {
                print!("{}", if x < height && x >= y { "*" } else { " " });
            }
            for _ in height + y..height * 2 - 1 {
                print!("*");
            }

            println!();
        }
    }
}