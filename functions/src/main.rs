fn main() {
    another_function();
    another_function2(5);
    print_labeled_measurement(5, 'h');

    // statements. Error: as it doesn't anything, you can't assign it to another variable
    /*
    - let y = 6 is a statement. It is assigning a value 6 to variable y but returning the value y or anything else.
    - Thus, when you try to assign the statement to another variable x; x doesn't have anything to receive or bind.
    - Therefore, the error.
    */
    // let x = (let y = 6);

    // expression and statement example
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let x1 = five();
    println!("The value of x1 is: {x1}");

    let x2 = plus_one(5);
    println!("The value of x2 is: {x2}");
}

// function without parameter
fn another_function() {
    println!("Another function");
}

// function with parameter
fn another_function2(x: i32) {
    println!("The value of x is: {x}")
}

// multiple parameters
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// function with return values
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    // x + 1; // throws an error as the expression turns into a statement by addding the semicolon. Thus, no return value.
    x + 1
}