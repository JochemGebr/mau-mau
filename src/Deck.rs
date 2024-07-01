use strum::IntoEnumIterator;
use crate::Card::{Card, Color};

use rand::thread_rng;
use rand::seq::SliceRandom;

pub struct Cards {
    cards: Vec<Card>
}

impl Cards {
    pub fn new() -> Cards {
        Cards {
            cards: Self::create_cards()
        }
    }

    fn create_cards() -> Vec<Card> {
        let mut cards = Vec::with_capacity(52);

        for color in Color::iter() {
            for level in 1..=13 {
                let card = Card { level, color };
                cards.push(card);
            }
        }
        cards.shuffle(&mut thread_rng());
        cards
    }

    pub fn get_random_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn get_random_cards(&mut self, n: usize) -> Vec<Cards> {
        self.cards.drain(0..5).as_slice()
    }

    pub fn reset(&mut self) {
        self.cards = Self::create_cards();
    }
}

#[cfg(test)]
mod tests {
    use crate::Cards::Cards;

    #[test]
    fn create_vec_of_cards() {
        let cards = Cards::create_cards();

        assert_eq!(52, cards.len());
    }
}