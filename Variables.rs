fn main() {
    println!("=== IMMUTABLE VARIABLES (Default) ===\n");
    
    let x = 5;
    println!("x = {}", x);
    
    // Uncommenting this will cause a COMPILE ERROR:
    // let x = 6;
    // println!("x is now: {}", x);
    
    println!("\n=== MUTABLE VARIABLES ===\n");
    
    let mut y = 10;
    println!("y starts as: {}", y);
    
    y = 15;
    println!("y changed to: {}", y);
    
    y = y + 5;
    println!("y is now: {}", y);
    
    println!("\n=== VARIABLE SHADOWING ===\n");
    
    // Shadowing: Declaring a new variable with the same name
    let z = 5;
    println!("First z: {}", z);
    
    let z = z + 1;  // Creates a NEW variable (shadows the old one)
    println!("Shadowed z: {}", z);
    
    let z = z * 2;  // Shadow again
    println!("Shadowed again: {}", z);
    
    // Shadowing vs Mutability
    // With shadowing, you can change the TYPE
    let spaces = "   ";  // String
    let spaces = spaces.len();  // Now it's a number!
    println!("Number of spaces: {}", spaces);
    
    // This wouldn't work with mut - uncommenting will cause error:
    // let mut spaces2 = "   ";
    // spaces2 = spaces2.len();  // ❌ Error! Can't change type
    
    println!("\n=== SCOPE AND LIFETIME ===\n");
    
    let outer = 1;
    println!("Outer variable: {}", outer);
    
    {  // New scope (inner block)
        let inner = 2;
        println!("Inside block - outer: {}, inner: {}", outer, inner);
        
        let outer = 3;  // Shadows the outer variable ONLY in this scope
        println!("Shadowed outer inside block: {}", outer);
    }  // inner variable is destroyed here
    
    println!("Back outside - outer is: {}", outer);  // Still 1!
    
    // println!("{}", inner);  // ❌ Error! inner doesn't exist here
    
    println!("\n=== CONSTANTS ===\n");
    
    // Constants are ALWAYS immutable and must have explicit type
    const MAX_POINTS: u32 = 100_000;  // Underscores for readability
    const PI: f64 = 3.14159;
    
    println!("Max points: {}", MAX_POINTS);
    println!("Pi: {}", PI);
    
    println!("\n=== TYPE INFERENCE ===\n");
    
    // Rust can usually figure out types automatically
    let inferred = 42;  // Rust knows this is a number
    println!("Inferred type: {}", inferred);
    
    // But sometimes you need to be explicit
    let explicit: i32 = 42;
    println!("Explicit type: {}", explicit);
    
    // When compiler can't infer:
    let parsed: u32 = "42".parse().expect("Not a number!");
    println!("Parsed: {}", parsed);
    
    // Without type annotation, this would fail - uncommenting causes error:
    // let parsed_fail = "42".parse().expect("Not a number!");  // ❌
    
    println!("\n=== NAMING CONVENTIONS ===\n");
    
    // Variables: snake_case
    let my_variable = 5;
    let user_count = 100;
    
    // Constants: SCREAMING_SNAKE_CASE
    const MAX_USERS: u32 = 1000;
    
    println!("my_variable: {}, user_count: {}, MAX_USERS: {}", 
             my_variable, user_count, MAX_USERS);
    
    println!("\n=== COMMON MISTAKES ===\n");
    
    // Mistake 1: Forgetting mut - uncommenting will cause error:
    let mistake1 = 5;
    // mistake1 = 6;  // ❌ Error: cannot assign twice to immutable variable
    
    // Fix: Use mut
    let mut fixed1 = 5;
    fixed1 = 6;  // ✅ Works!
    println!("Fixed: {}", fixed1);
    
    // Mistake 2: Unused variables (compiler warning)
    let _unused = 10;  // Prefix with _ to silence warning
    
    // Mistake 3: Uninitialized variables - uncommenting will cause error:
    // let uninitialized;
    // println!("{}", uninitialized);  // ❌ Error: can't use before assigning
    
    // Fix: Initialize before use
    let initialized;
    initialized = 42;
    println!("Initialized: {}", initialized);
    
    println!("\n✅ All demonstrations complete!");
}

// ============================================
// KEY TAKEAWAYS:
// ============================================
// 1. Variables are immutable by default (safety first!)
// 2. Use 'mut' when you need to change a value
// 3. Shadowing lets you reuse names and change types
// 4. Constants are for values that never change
// 5. Rust's compiler helps you write correct code
// 6. Scope determines where variables are valid
//
// MENTAL MODEL:
// Think of immutable variables as "labels" on boxes
// that you can't move. Mutable variables are boxes
// where you can swap the contents.
/
