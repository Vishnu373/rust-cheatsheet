fn main() {
    // Immutable variable / default
    let x = 6;
    println!("The value of x is: {x}");

    x = 5;
    println!("The value of x is; {x}");

    // Mutable variable
    let mut x = 6;
    println!("The value of x is: {x}");

    x = 5;
    println!("The value of x is: {x}");

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Shadowing
    let x = 5;
    println!("Intial value of x is: {x}");

    let x = x + 1;
    println!("After shadowing, the value of x is: {x}");

    {
        // Shadowed but the scope is limited within the curly brackets
        let x = x * 2;
        println!("The value of x in inner scope is: {x}");
    }

    println!("The value of x in outer scope is: {x}");

    // Shadowing with same variable name but type change
    let spaces = "   ";
    println!("The value of spaces is: {spaces}");
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");
}
