const THREE_DAYS_IN_SECONDS: u64 = 3 * 24 * 60 * 60;

fn main() {
    test_variables();
    test_spaces();
    test_floats();
    test_operations();
    test_booleans();
    test_chars();
    let string = "Hello, world!";
    test_strings(string);
    test_tuples();
    test_arrays();
    test_flow_control();
    test_loop();
    test_while();
    test_for();
}

fn test_variables() {
    let mut x: i32 = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    println!("Three days in seconds: {THREE_DAYS_IN_SECONDS}");
}

fn test_spaces() {
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The spaces are: {spaces}");
}

fn test_floats() {
    let x = 0.1;
    let y = 0.2;
    let sum = x + y;
    println!("The sum of x and y is: {sum}");
}

fn test_operations() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    println!("The sum of 5 and 10 is: {sum}");
    println!("The difference of 95.5 and 4.3 is: {difference}");
    println!("The product of 4 and 30 is: {product}");
    println!("The quotient of 56.7 and 32.2 is: {quotient}");
    println!("The floored quotient of 2 and 3 is: {floored}");
    println!("The remainder of 43 and 5 is: {remainder}");
}

fn test_booleans() {
    let t = true;
    println!("The value of t is: {t}");
    let f: bool = false; // with explicit type annotation
    println!("The value of f is: {f}");
}

fn test_chars() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("The value of c is: {c}");
    println!("The value of z is: {z}");
    println!("The value of heart_eyed_cat is: {heart_eyed_cat}");
}

fn test_strings(string: &str) {
    println!("The string is: {string}");
}

fn test_tuples() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");
    println!("The value of x is: {x}");
}

fn test_arrays() {
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("The value of first is: {first}");
    println!("The value of second is: {second}");
}

fn test_flow_control() {
    let x = 5;
    let y = if x == 5 { 10 } else { 15 };
    println!("The value of y is: {y}");
}

fn test_loop() {
    let mut x = 0;
    'test_1: loop {
        let mut y = 10;
        loop {
            println!("The value of y is: {y}");
            println!("The value of x is: {x}");
            y = y - 1;
            if y == 8 {
                break;
            }

            if x == 2 {
                break 'test_1;
            }
        }
        if x == 10 {
            break;
        }
        x = x + 1;
    }
}

fn test_while() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn test_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
