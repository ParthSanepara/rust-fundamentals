# rust-fundamentals
Coursera Rust Fundamentals course

### Rust course referance Repor

- [week 1](https://github.com/alfredodeza/rust-setup) 
- [week 2](https://github.com/alfredodeza/rust-fundamentals)
- [week 3](https://github.com/alfredodeza/rust-structs-types-enums/)
- [week 4](https://github.com/alfredodeza/applied-rust)

- [Fragile ref. example](https://github.com/mitsuhiko/fragile)

# Notes:

### Create new project / lib 
```bash
# Create a new binary application
cargo new project_name

# Create a new library
cargo new --lib library_name

# Project structure
project_name/
├── Cargo.toml       # Project metadata and dependencies
└── src/
    └── main.rs      # Entry point for binary projects
    
library_name/
├── Cargo.toml       # Project metadata and dependencies
└── src/
    └── lib.rs       # Entry point for library projects
```

### build and run project
```bash
# Compile the project
cargo build

# Compile and run the project
cargo run

# Check if the code compiles without producing an executable
cargo check

# Build with optimizations for release
cargo build --release

# Run tests
cargo test

# The compiled binary will be in target/debug/ or target/release/
```

### Variable Assignment in Rust

```rust
fn main() {
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
    let count = count + 1;              // New variable shadows previous one
    
    println!("My name is {}", name);
    println!("a: {}, b: {}, c: {}", a, b, c);
    println!("d: {}", d);
}
```

### Key Concepts

| Feature | Description | Example |
|---------|-------------|---------|
| **Immutability** | Variables are immutable by default | `let x = 5;` |
| **Mutability** | Use `mut` to allow reassignment | `let mut x = 5;` |
| **Type Inference** | Compiler determines types automatically | `let x = 5;` → i32 inferred |
| **Type Annotation** | Explicitly specify variable type | `let x: f64 = 5.0;` |
| **Shadowing** | Reuse variable names by creating new bindings | `let x = 5; let x = x + 1;` |
| **Constants** | Immutable values with required type annotation | `const MAX: u32 = 100;` |
| **Compound Assignment** | Modify and reassign in one step | `x += 1;` equivalent to `x = x + 1` |

### Memory Safety Through Assignment

Rust's assignment rules enforce memory safety by preventing:
- Data races through controlled mutability
- Null pointer dereferencing through Option<T>
- Memory leaks through ownership system
- Use-after-free