/*
// Repeating Code with loop
fn main() {
    loop {
        println!("again!");
    }
}
*/

/*
// If you have loops within loops,
// break and continue apply to the innermost loop at that point.
// You can optionally specify a loop label on a loop that
// we can then use with break or continue to specify that
// those keywords apply to the labeled loop instead of the innermost loop
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}
*/

/*
// Returning Values from Loops
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // return a value from a loop
        }
    };

    println!("The result is {}", result);
}
*/

/*
// Conditional Loops with while
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
*/

/*
// Looping Through a Collection with for
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }
}
*/

// another for loop usage example
fn main() {
    for number in 1..4 {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}