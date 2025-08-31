fn main() {
    println!("Hello, world!");

  // Immutable variables (default)
    let name = "Parth!";                // String type inferred
    let a = 100.1;                      // f64 type inferred
    let b = 25.2;
    let c = a / b;                      // c = 3.9722...
    
    // Type annotations (optional but sometimes necessary)
    let e: i32 = 42;                    // Explicitly i32
    let f: f32 = 3.14;                  // Explicitly f32
    
    // Mutable variables
    let mut d = 10;                     // Mutable integer
    d = 25;                             // Valid reassignment
    // d += 5;                          // Compound assignment (adds 5 to d)
    
    // Constants (uppercase by convention)
    const MAX_POINTS: u32 = 100_000;    // Type annotation required
    
    // Shadowing
    let count = 5;
    println!("Initial count: {}", count);
    let count = 7.5;              // New variable shadows previous one
    
    println!("My name is {}", name);
    println!("a: {}, b: {}, c: {}", a, b, c);
    println!("d: {}", d);
    println!("e: {}, f: {}", e, f);
    println!("MAX_POINTS: {}", MAX_POINTS);
    println!("count: {}", count);



    let process = true;

    if process {
        println!("Processing...");
    } else {
        println!("Not processing.");
    }

    let cmp_data = 99;

    if cmp_data < 100 {
        println!("Data is less than 100");
    } else if cmp_data == 100 {
        println!("Data is exactly 100");
    } else {
        println!("Data is greater than 100");
    }

    let mut x = 0;
    loop {
        println!("Looping...");
        x += 1;
        if x == 5 {
            println!("Breaking the loop.");
            break;
        }
    }

    println!("exit!");
}
