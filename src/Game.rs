use crate::Settings::Settings;
use crate::Deck::Deck;
use crate::Players::Players;

pub struct Game {
    settings: Settings,
    players: Players,
    deck: Deck,
}

impl Game {
    pub fn new(settings: Settings) -> Self {
        Game {
            settings: settings,
            players: Players::new(),
            deck: Deck::new(),
        }
    }

    pub fn get_players_mut(&mut self) -> &mut Players {
        &mut self.players
    }

    pub fn get_players(&self) -> &Players {
        &self.players
    }

    pub fn get_deck_mut(&mut self) -> &mut Deck {
        &mut self.deck
    }

    pub fn get_deck(&self) -> &Deck {
        &self.deck
    }

    pub fn start(&self) {

    }
}

#[cfg(test)]
mod tests {
    use crate::Game::Game;
    use crate::Settings::Settings;

    #[test]
    fn make_new_game() {
        let settings = Settings::new();
        let game = Game::new(settings);
    }

}