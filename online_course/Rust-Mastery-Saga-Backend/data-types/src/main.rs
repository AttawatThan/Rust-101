use std::fmt::format;

fn main() {
    let x: i32 = 1;
    let y: f64 = 0.5;
    let z: i32 = x + y as i32;

    let msg: String = String::from("Hello, World!");
    let msg2: String = "Hello, World!".to_string();
    let msg3: String = format!("Point: {}, {}", {x}, {y});

    let msg4: &'static str = "Hello, World";

    println!("{}", msg3);

     
}
