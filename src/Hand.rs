use crate::Card::Card;
use crate::Deck::Deck;

pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Hand {
        Hand {
            cards: Vec::new(),
        }
    }

    pub fn take_cards(mut self, amount: usize, deck: &mut Deck) {
        for _ in 0..amount {
            self.cards.push(deck.draw_card());
        }
    }

    pub fn add_cards(&mut self, cards: &mut Vec<Card>) {
        self.cards.append(cards);
    }

    pub fn get_cards(&self) -> &Vec<Card> {
        &self.cards
    }
}