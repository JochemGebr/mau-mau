use std::io::{self};

use crate::{card::Card, pile::Pile};

pub trait Player {
    fn new(name: &str) -> dyn Player;

    fn get_name(&self) -> &String {
        &self.name
    }
}

pub trait HandedPlayer {
    fn get_hand(&self) -> &Pile;
    fn get_hand_mut(&mut self) -> &mut Pile;

    fn select_card(&mut self) -> Option<Card> {
        println!("Your hand: {}", self.get_hand().get_drawing());
        println!("Select a card to play by typing its index: ");
        let mut input_line = String::new();
        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");
        let index: usize = input_line.trim().parse().expect("Input not an integer");
        self.get_hand_mut().take_card(index)
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
