use crate::hand::Hand;

pub struct Player {
    name: String,
    points: u32,
    hand: Hand
}

impl Player {
    pub fn new(name: &str) -> Player {
        Player {
            name: name.to_string(),
            points: 0,
            hand: Hand::new(),
        }
    }

    pub fn add_points(&mut self, points: u32) {
        self.points += points;
    }

    pub fn get_hand(&self) -> &Hand {
        &self.hand
    }

    pub fn get_hand_mut(&mut self) -> &mut Hand {
        &mut self.hand
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}