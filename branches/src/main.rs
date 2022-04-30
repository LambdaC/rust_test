/*
// if expression
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
*/

/*
// error example
// It’s also worth noting that the condition in this code must be a bool.
// If the condition isn’t a bool, we’ll get an error.
fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
}
*/

/*
// Handling Multiple Conditions with else if
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
*/

// Using if in a let Statement
// Because if is an expression, we can use it on the right side of 
// a let statement to assign the outcome to a variable
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}
