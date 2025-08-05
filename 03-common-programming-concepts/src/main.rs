fn main() {
    // Quando si dichiara una costante è necessario specificare il tipo
    const Y: i32 = 10;
    println!("Y = {}", Y);

    // Dichiarazione di una variabile mutabile
    let mut x = 5;
    println!("X = {}", x);
    x = 6;
    println!("X = {}", x);

    //Dichiarazione di una tupla
    let tup: (i32, f64, String) = (1, 2.0, String::from("Hello"));
    println!("TUPLE = {:?}", tup);

    // Dichiarazione di un array
    let arr: [i32; 3] = [1, 2, 3];
    println!("ARRAY = {:?}", arr);

    my_function();

    // Chiamata alla funzione sum
    let result = sum(5, 10);
    println!("Sum of 5 and 10 is: {}", result);

    //Espressioni di controllo
    if 10 > 20 {
        println!("10 is greater than 20");
    } else {
        println!("10 is not greater than 20");
    }

    // Ciclo for
    for i in 0..5 {
        println!("Iteration: {}", i);
    }

    // Ciclo while
    let mut count = 0;
    while count < 5 {
        println!("Count: {}", count);
        count += 1;
    }

    // Ciclo loop
    let mut n = 0;
    loop {
        if n >= 5 {
            break;
        }
        println!("Looping: {}", n);
        n += 1;
    }

    // Loop che restituisce il valore di una variabile
    let mut m = 0;
    let result_loop = loop {
        if m >= 5 {
            break m * 2; // Restituisce il valore di m moltiplicato
        }
        m += 1;
    };

    println!("Result from loop: {}", result_loop);
}

//dichiarazione di una funzione
fn my_function() {
    // Questa funzione non fa nulla
    // Puoi aggiungere il tuo codice qui
    println!("This is my function.");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}
