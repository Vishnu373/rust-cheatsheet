fn main() {
    // comment out line 3 and 4 alternatively to see if and else block getting executed respectively.
    let number = 3;
    let number = 7;

    if number < 5 {
        println!("Condition was true");
    }

    else {
        println!("Condition was false");
    }

    // will throw an error as the condition isn't a boolean
    /*
    let number1 = 3;

    if number1 {
        println!("Number was three");
    }
    */

    // multiple if statements with else if
    let number2 = 6;

    if number2 % 4 == 0 {
        println!("number is divisible by 4");
    } else if number2 % 3 == 0 {
        println!("number is divisible by 3");
    } else if number2 % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // using if in a let statement
    let condition = true;
    let number3 = if condition { 5 } else { 6 };
    // let number4 = if condition { 5 } else { "six" }; // will throw an error
    // let number5 = if condition { "five" }; // else block is required this case.
        
    println!("The value of number3 is: {number3}");
}
