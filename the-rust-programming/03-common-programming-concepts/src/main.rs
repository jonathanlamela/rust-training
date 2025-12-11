fn main() {
    // When declaring a constant, it is necessary to specify the type
    const Y: i32 = 10;
    println!("Y = {}", Y);

    // Declaration of a mutable variable
    let mut x = 5;
    println!("X = {}", x);
    x = 6;
    println!("X = {}", x);

    // Declaration of a tuple
    let tup: (i32, f64, String) = (1, 2.0, String::from("Hello"));
    println!("TUPLE = {:?}", tup);

    // Declaration of an array
    let arr: [i32; 3] = [1, 2, 3];
    println!("ARRAY = {:?}", arr);

    my_function();

    // Call to the sum function
    let result = sum(5, 10);
    println!("Sum of 5 and 10 is: {}", result);

    // Control expressions
    if 10 > 20 {
        println!("10 is greater than 20");
    } else {
        println!("10 is not greater than 20");
    }

    // For loop
    for i in 0..5 {
        println!("Iteration: {}", i);
    }

    // While loop
    let mut count = 0;
    while count < 5 {
        println!("Count: {}", count);
        count += 1;
    }

    // Loop with break
    let mut n = 0;
    loop {
        if n >= 5 {
            break;
        }
        println!("Looping: {}", n);
        n += 1;
    }

    // Loop that returns the value of a variable
    let mut m = 0;
    let result_loop = loop {
        if m >= 5 {
            break m * 2; // Returns the value of m multiplied
        }
        m += 1;
    };

    println!("Result from loop: {}", result_loop);
}

// Declaration of a function
fn my_function() {
    // This function does nothing
    // You can add your code here
    println!("This is my function.");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}
