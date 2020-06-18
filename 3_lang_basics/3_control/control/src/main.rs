fn main() {
    // if statements in Rust seem pretty normal
    let condition = true;
    let number = if condition { 6 } else { 5 };

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // "while true" loops are done using the keyword "loop" in Rust, and they can return values
    let mut counter: i32 = 0;

    let result: i32 = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // Rust also has the standard "while" loop
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("while number result: {0}", number);

    // and it has the python-like for loop
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    for element in a.iter() {
        println!("the value is: {}", element * element);
    }

    // and a more JS-esque for loop
    for number in (1..4).rev() {
        println!("straight for loop: {}!", number);
    }
}

