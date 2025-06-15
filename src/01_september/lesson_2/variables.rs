fn main() {
    // numeric values
    {
        // signed ints
        let a: i8 = 1;      // signed 8-bit int     (-128 - 127)
        let b: i16 = 1;     // signed 16-bit int    (-32,768 - 32,767)
        let c: i32 = 1;     // signed 32-bit int    (-2,147,483,648 - 2,147,483,647)
        let d: i64 = 1;     // signed 64-bit int    (-9,223,372,036,854,775,808 - 9,223,372,036,854,775,807)
        let e: i128 = 1;    // signed 128-bit int   (-1.7014118346046923173168730371588e+38 - 1.7014118346046923173168730371588e+38 - 1)

        // unsigned ints
        let a_u: u8 = 1;     // unsigned 8-bit int   (0 - 255)
        let b_u: u16 = 1;    // unsigned 16-bit int  (0 - 65,535)
        let c_u: u32 = 1;    // unsigned 32-bit int  (0 - 4,294,967,295)
        let d_u: u64 = 1;    // unsigned 64-bit int  (0 - 18,446,744,073,709,551,615)
        let e_u: u128 = 1;   // unsigned 128-bit int (0 - 3.4028236692093846346337460743177e+38 - 1)
    }
    
    // numeric values with a decimal point
    {
        // all floating-point types are signed
        let a: f32 = 1.1;   // signed 32-bit float
        let b: f64 = 1.1;   // signed 64-bit float (default for floats)
        
        // alternatively state the float size in the assigned value (i.e. f32, f64)
        let a_alt = 1.1f32;
        let b_alt = 1.1f64;
        
        // 'double' typically refers to f64 (the signed 64-bit float)
    }
    
    // true/false (boolean) values
    {
        let a: bool = true;
        let b: bool = false;
    }
    
    // text and letters
    {
        let a: char = 'a';                                  // 4 bytes in size
        let b: &str = "Hello, World!";                      // String Slice / Borrowed String - it's a part of a String stored in the compiled ROM and we're getting a reference to a slice of that String, but we don't own it.
        let c: String = "Hello, World!".to_owned();         // "Hello, World!" is actually &str and needs to be converted to String. This can be done through .to_owned() or .to_string()
        let d: String = String::from("Hello, World!");   // Also possible, usually preferred for string literals.
        
        // &str is just a pointer and so is much cheaper to work with
        // String creates a new owned space in heap memory
    }
}