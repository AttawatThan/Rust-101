fn main() {
    // constant variable
    const THREE_HOURS_IN_SECONDS: i32 = 60 * 60 * 3;
    println!("The value of constant is: {}", THREE_HOURS_IN_SECONDS);

    // immutable variable
    let y = 8;
    println!("The value of y is: {y}");
    // mutable variable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // shadowing variable
    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    // use-case of used shadowing variable --> can reuse same with same name
    let space = "       ";
    let space = space.len();
    println!("The value of space is: {space}")
}
