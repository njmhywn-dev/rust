struct Crabby {
    name: String,
    health: u8,
}

impl  Crabby {
    fn take_damage(&mut self, damage: u8) {
        self.health = self.health.saturating_sub(damage);
    }

    fn healing(&mut self, heal: u8) {
        if self.health + heal >= 100 {
            self.health = 100;
            return;
        }
        self.health += heal;
    }
    
}

fn main() {
    let mut crabby: Crabby = Crabby {
        name: "Crabby".to_string(),
        health: 100,
    };

    crabby.healing(0);
    println!("{} health : {}", crabby.name, crabby.health);

    crabby.take_damage(200);
    println!("{} health : {}", crabby.name, crabby.health);

    
}