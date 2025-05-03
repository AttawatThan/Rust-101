fn another_function() {
    println!("This is another_function");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    // How to caled the function.
    another_function();

    print_labeled_measurement(5,'h');

    // Example function with no return value
    let x: i32 = five();
    println!("The values of x is: {x}");

    // Example function with return value
    let y: i32 = plus_one(5);
    println!("The values of y is: {y}");


    // Expression
    let w: i32 = {
        let x: i32 = 3;
        x + 1 // Not assign value to x, therefore it can return
    };
    println!("The values of w is: {w}")
}

