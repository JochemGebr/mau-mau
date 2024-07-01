use strum::IntoEnumIterator;
use crate::Card::{Card, Color};

use rand::thread_rng;
use rand::seq::SliceRandom;

pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
    pub fn new() -> Deck {
        Deck {
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

    pub fn draw_card(&mut self) -> Card {
        self.cards.pop().unwrap()
    }

    pub fn draw_cards(&mut self, amount: usize) -> Vec<Card> {
        let start_index = self.cards.len().saturating_sub(amount);
        let cards = self.cards.drain(start_index..).collect();

        cards
    }

    pub fn reset(&mut self) {
        self.cards = Self::create_cards();
    }
}

#[cfg(test)]
mod tests {
    use crate::Deck::Deck;

    #[test]
    fn create_vec_of_cards() {
        let cards = Deck::create_cards();

        assert_eq!(52, cards.len());
    }

    #[test]
    fn random_cards_dont_repeat() {
        let mut cards = Deck::new();
        let selection = cards.draw_cards(52);

        //assert_eq!(52, selection.iter().);
    }

    #[test]
    fn many_times_look_at_that_performance_this_would_be_useful_when_everyone_in_the_world_wants_to_play_a_game_at_the_same_time() {
        for _ in 0..1000000 {
            Deck::new();
        }
    }
}