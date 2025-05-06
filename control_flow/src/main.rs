fn main() {
    // if Expressions
    // if-else
    let number: i32 = 7;

    if number < 5 {
        println!("condition was true");
    }
    else {
        println!("condition was false");
    }

    // multiple conditions
    let number: u8 = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 4 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // if in let statement
    let condition: bool = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");


    //Repetition with Loops

    // 1. loop
    // loop with break
    let mut counter = 0;
    loop {
        println!("again !");
        counter += 1;
        if counter == 10 {
            break;
        }
    }
    
    // loop with return value
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
    println!();

    // loop labels to Disambiguate between multiple loops
    let mut count: i32 = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");

            // All condition innermost loop
            
            // inner loop condition to exit
            if remaining == 9 {
                break;
            }
            
            // outer loop condition to exit (name loop labeled 'counting_up)
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
    println!();

    // 2. Loops with while
    let mut number = 3;
    while number != 0 {
        println!("number: {number}");

        number -= 1;
    }
    println!("LIFTOFF!!!");
    println!();
    
    // 3. Loops with for
    // loop over the elemnents of a collection, such as an array
    // - can also use while construct to loop
    let a: [u8; 5] = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
    println!();

    // - can use a for loop
    for element in a {
        println!("the value is: {element}");
    }
    println!();

    for number in (1..4).rev() {
        println!("number: {number}");
    }
    println!("LIFTOFF!!!");


}
