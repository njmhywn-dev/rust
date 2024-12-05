enum CrabbyState {
    Fighting,
    Collecting(u32),
    Defending,
}

impl CrabbyState {
    fn current_state(&self) {
        match self {
            CrabbyState::Fighting => println!("Crabby is Fighting"),
            CrabbyState::Collecting(collection) => println!("Crabby is have {} collection", collection),
            CrabbyState::Defending => println!("Crabby is Defending"),
        }
    }
    
}

fn main() {
    let fighting = CrabbyState::Fighting;
    let collecting = CrabbyState::Collecting(10);
    let defending = CrabbyState::Defending;

    fighting.current_state();
    collecting.current_state();
    defending.current_state();
    
}
