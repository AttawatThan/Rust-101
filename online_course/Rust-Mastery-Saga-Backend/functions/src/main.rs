fn crabby_tasks(task: &str, time: i32) -> String {
    format!("Crabby has succesfully done {} in {} minutes!", task, time)
}

fn main() {
    let result: String = crabby_tasks("gathering coins", 12);
    println!("{}", result);

    // Add more function calls here!
    let result_2 = crabby_tasks("cooking", 30);
    println!("{}", result_2);
    

    let result_3: String = crabby_tasks("hunting", 8);
    println!("{}", result_3);


}

