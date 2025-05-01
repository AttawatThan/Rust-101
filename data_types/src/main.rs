use std::io;

fn main() {
    
    // Scalar Types
    // - Integer
    let x_int: i32 = 12; // Signed int, Default is i32 type
    let y_int: u32 = 80; // Unsigned int
    println!("The value of x_int is: {x_int}");
    println!("The value of y_int is: {y_int}");
    println!();

    // - Floating-Points
    let x_float: f32 = 0.1234;
    let y_float: f64 = 0.2345; // Default is f64 type
    println!("The value of x_float is: {x_float}");
    println!("The value of y_float is: {y_float}");
    println!();

    // - Boolean
    let t = true;
    let f: bool = false;
    println!("The value of t is: {t}");
    println!("The value of f is: {f}");
    println!();
    
    // - Character
    let c = 'z';
    let z: char = 'Z';
    let face_eyed_cat: char = '🐱';
    println!("The value of c is: {c}");
    println!("The value of z is: {z}");
    println!("The value of face_eyed_cat is: {face_eyed_cat}");
    println!();


    // Compound Types
    // - Tuple
    let tup: (i32, f64, u8, char) = (500, 7.0, 4, 'z');
    // accessing a tuple
    let first_element:i32 = tup.0;
    println!("The value of first_element is: {first_element}");
    // Destructuring
    let (w, _x, _y, _z) = tup;
    println!("The value of w is: {w}");
    println!();

    // Array
    let a: [i32; 5] = [1, 2, 3,4 ,5];
    let _b: [i32; 5] = [3; 5]; // equal to [3, 3, 3, 3, 3]
    // accessing a array
    let first_elemet: i32 = a[0];
    println!("The value of z is: {first_elemet}");

    // Invalid Array Element Access
    let a: [u32; 5] = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is : {element}.");
    
}
