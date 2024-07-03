use std::io::{self};

use crate::{card::Card, pile::Pile};

pub struct Player {
    name: String,
    points: u32,
    hand: Pile,
}

impl Player {
    pub fn new(name: &str) -> Player {
        Player {
            name: name.to_string(),
            points: 0,
            hand: Pile::new(),
        }
    }

    pub fn add_points(&mut self, points: u32) {
        self.points += points;
    }

    pub fn get_hand(&self) -> &Pile {
        &self.hand
    }

    pub fn get_hand_mut(&mut self) -> &mut Pile {
        &mut self.hand
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn select_card(&mut self) -> Option<Card> {
        println!("Your hand: {}", self.hand.get_drawing());
        println!("Select a card to play by typing its index: ");
        let mut input_line = String::new();
        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");
        let index: usize = input_line.trim().parse().expect("Input not an integer");
        self.hand.take_card(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_name() {
        let player = Player::new("Alice");
        assert_eq!("Alice", player.get_name());
    }

    #[test]
    fn add_points() {
        let mut player = Player::new("Alice");
        player.add_points(10);
        assert_eq!(10, player.points);
    }
}
