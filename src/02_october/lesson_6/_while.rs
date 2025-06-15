use rand::{rng, Rng};

fn main() {
    // even numbers from 0 to 1000
    {
        for i in (0..=100).step_by(2) {
            print!("{} ", i)
        }
        println!();
    }

    // dice roll
    {
        let mut roll: i32;
        loop {
            roll = rng().random_range(1..=6);
            println!("Rolled {}.", roll);
            if roll == 6 { break; }
        }
        println!("Congrats!")
    }
    
    // labeled expression with break and a returned value, nested blocks
    {
        let result: i8 = 'block_a: {
            let nested_result: &str = match rng().random_range(0..=100) {
                0..=50 => 'nested_block: {
                    let nested_rand = rng().random_range(50..=100);
                    if nested_rand % 2 == 0 {
                        break 'nested_block "Nested Even"
                    } else {
                        if nested_rand <= 75 {
                            break 'block_a 2;
                        } else {
                            break 'nested_block "Nested Odd and over 75";
                        }
                    }
                }
                _ => "Default"
            };
            
            if nested_result == "Nested Even" {
                break 'block_a 1
            } else if nested_result == "Nested Odd and over 75" {
                break 'block_a 3;
            } else {
                break 'block_a 0;
            }
        };
        
        println!("Result code is {}!", result)
        /*
            0       'nested_block   resulted in         "Default"
            1       'nested_block   resulted in         "Nested Even",
            2       'nested_block   called break on     'block_a                    with value 2
            3       'nested_block   resulted in         "Nested Odd and over 75"
         */
    }
}