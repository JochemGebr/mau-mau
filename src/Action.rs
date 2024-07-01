use crate::Game::Game;

pub trait Action {
    fn execute(&self, game: &mut Game);
}

pub struct TakeCards {
    pub amount: usize,
    pub next_player: usize,
}

impl Action for TakeCards {
    fn execute(&self, game: &mut Game) {
        let mut cards_to_add = game.get_deck_mut().draw_cards(self.amount);
        let player = game.get_players_mut().get_next_player_at_mut(self.next_player);
        let hand = player.get_hand_mut();
        hand.add_cards(&mut cards_to_add);
    }
}

#[cfg(test)]
mod tests {
    use crate::Game::Game;
    use crate::Settings::Settings;
    use crate::Player::Player;
    use crate::Action::{Action, TakeCards};

    #[test]
    fn take_two_cards() {
        let settings = Settings::new();
        let mut game = Game::new(settings);
        let mut players = game.get_players_mut();
        let player1 = Player::new("Piet");
        players.add_player(player1);
        let player2 = Player::new("Jan");
        players.add_player(player2);
        let player3 = Player::new("Klaas");
        players.add_player(player3);

        let action = TakeCards {
            amount: 2,
            next_player: 1,
        };
        action.execute(&mut game);
        
        let player2 = game.get_players().get_next_player_at(1);
        assert_eq!(player2.get_hand().get_cards().len(), 2);
    }
}