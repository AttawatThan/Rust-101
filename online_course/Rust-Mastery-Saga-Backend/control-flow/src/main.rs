fn main() {
    // 1. Decision Time! (if-else)
    let weather: &'static str = "rainy";

    if weather == "sunny" {
        println!("Crabby, will cross the river by swimming!");
    } else if weather == "rainy" {
        println!("Crabby, will build a bridge to stay dry.")
    } else if weather == "stormy" {
        println!("Crabby, will wait for better weather.")
    }
    
    // 2. Enemy List. (match)
    let monster: &'static str = "dragon";
    match monster {
        "goblin" => println!("Crabby uses his rusty sword to attack"),
        "troll" => println!("Crabby sets a trap!"),
        "dragon" => println!("Crabby runs for cover!"),
        _ => println!("Crabby is confused...")
    }

    //3. Crabby repeated Task. (loop)
    let mut wood: u8 = 0;

    loop {
        wood += 1;
        println!("Crabby's gathered {} pieces of wood.", wood);

        if wood == 10 {
            println!("Crabby finished the boat!");
            break;
        }
    }

}
