pub struct Character {
    name: String,
    health: u16,
    inventory: Vec<usize>,
}

impl Character {
    pub fn new(player_name: &str) -> Character {
        Character {
            name: player_name.to_string(),
            health: 100,
        }
    }

    pub fn get_name(self) -> String {
        self.name
    }

    pub fn get_health(self) -> u16 {
        self.health
    }

    pub fn is_dead(&self) -> bool {
        if self.health <= 100 { true } else { false }
    }

    pub fn attack(character: &Character) -> bool {
        return true;
    }
}
