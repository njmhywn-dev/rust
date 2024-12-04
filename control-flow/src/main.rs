
// Example 1
//If it’s "sunny", Crabby will cross the river by swimming! 
//If it’s "rainy", Crabby will build a bridge to stay dry. 
//If it’s "stormy", Crabby will wait for better weather.
// fn main() {
//     let weather = "sunny";

//     if weather == "sunny" && weather == "rainy" {
//         println!("Crabby will cross the river by swimming!");
//     }else if weather == "rainy" {
//         println!("Crabby will build a bridge to stay dry!");
//     }else if weather == "stormy" {
//         println!("Crabby will wait for better weather!");
//     }
// }

// Example 2
// If Crabby encounters a "goblin", he uses his rusty sword to attack. 
// For a "troll", Crabby sets a trap! 
// If he meets a "dragon", Crabby runs for cover! 
// Everything else? Crabby is confused...
// fn main() {
//     let enemy = "dragon";
    
//     match enemy {
//         "goblin" => println!("Crabby uses his rusty sword to attack."),
//         "troll" => println!("Crabby sets a trap!"),
//         "dragon" => println!("Crabby runs for cover!"),
//         _ => println!("Crabby is confused...")
//     }
// }

// Example 3
// Use a loop that increases the amount of wood by 1 each time.
// When Crabby’s gathered 10 pieces, print "Crabby finished the boat
fn main() {
    let mut woods = 0;

    loop {
        woods += 1;
        println!("Crabby have a wood {}.", woods);

        if woods >= 10 {
            println!("Crabby finished the boat");
            break;
        }
    }
}
