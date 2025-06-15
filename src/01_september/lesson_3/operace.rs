mod variables_cv;

fn main() {
    // Arithmetic Operators
    {
        let sum = 100 + 200;        // 300
        let diff = sum - 200;       // 300 - 200 = 200
        let ratio = diff / diff;    // is 1.5 as a float but only 1 as an int, the decimal point is cut off (dividing two ints gives out another int)
        let mut prod = sum * diff;  // by default, variables cannot be changed. Add 'mut' to make them mutable.
        
        prod += 1;      // Rust doesn't have v++, ++v, v--, --v.
        prod -= 1;
        prod /= 2;
        prod *= 3;
        prod %= 5;    // modulo operator

        println!("sum {}, diff {}, ratio {}, prod {}", sum, diff, ratio, prod);
    }
    
    // (Non)equality operators
    {
        let a = 1;
        let b = 2;

        dbg!(a == b);     // a equals b
        dbg!(a != b);     // a doesn't equal b
        dbg!(a > b);
        dbg!(a < b);
        dbg!(a >= b);
        dbg!(a <= b);
    }
    
    // Logic operators
    {
        // AND
        let a = 50;
        let min = 0;
        let max = 100;
        
        let in_interval : bool = a > min && a < max;
        
        // OR
        let valid_id_1 = 156781;
        let valid_id_2 = 982181;
        let id = 42;
        
        let is_valid: bool = id == valid_id_1 || id == valid_id_2;
        
        // NOT
        let is_not_valid: bool = !is_valid;
    }
}