fn main() {
    // integers
    let num: i32 = 45;
    println!("The value of num is: {num}");

    // floating point
    let x = 2.0; // f64
    let y: f32 = 4.8;

    println!("The value of x is {x}");
    println!("The value of y is {y}");

    // boolean
    let t = true;
    let f: bool = false;

    println!("The value of t is: {t}");
    println!("The value of f is: {f}");

    // char type
    let c = 'z';
    let z: char = 'Z'; 
    let heart_eyed_cat = '😻';

    println!("The value of c is: {c}");
    println!("The value of z is: {z}");
    println!("The value of heart_eyed_cat is: {heart_eyed_cat}");

    // addition
    let sum = 5 + 10;

    println!("The value of sum is: {sum}");

    // subtraction
    let difference = 95.5 - 4.3;

    println!("The value of difference is: {difference}");

    // multiplication
    let product = 4 * 30;

    println!("The value of product is: {product}");

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;

    println!("The value of quotient is: {quotient}");
    println!("The value of truncated is: {truncated}");

    // remainder
    let remainder = 43 % 5;

    println!("The value of remainder is: {remainder}");

    tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);

    // Destructing to print out individual elements
    let (x, y, z) = tup;

    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");

    // Indexing to print out individual elements.
    println!("The value of first element in tup is: {}", tup.0);
    println!("The value of second element in tup is: {}", tup.1);
    println!("The value of third element in tup is: {}", tup.2);

    // empty tuple
    let empty_tup: () = ();
    let empty_tup = ();
    
    // mutable tuple
    let mut x: (i32, i32) = (1, 2);
 
    println!("The value of first element in tup is: {}", x.0);
    println!("The value of second element in tup is: {}", x.1);
 
    x.0 = 0;
    x.1 += 5;

    println!("After updating the elements.");
    println!("The value of first element in tup is: {}", x.0);
    println!("The value of second element in tup is: {}", x.1);

    // Array
    let arr = [1, 2, 3, 4, 5];
    let arr2: [i32; 5] = [1, 2, 3, 4, 5]; // type - i32, length of the array - 5;
    let arr3 = [3; 5]; // An array of length 5 with all elements as 3. [3, 3, 3, 3, 3]

    // Indexing
    let first = arr[0];
    let second = arr[1];

    println!("The value of first is: {first}");
    println!("The value of second is: {second}");


}
