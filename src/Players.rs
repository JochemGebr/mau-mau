use crate::player::Player;
use crate::pile::Pile;

pub struct Players {
    current_player_index: usize,
    players: Vec<dyn Player>,
}

impl Players {
    pub fn new() -> Players {
        Players {
            current_player_index: 0,
            players: vec![],
        }
    }

    pub fn deal(&mut self, deck: &mut Pile, number_of_cards: usize) {
        for player in self.players.iter_mut() {
            player.get_hand_mut().take_cards_from(deck, number_of_cards);
        }
    }

    pub fn add_player(&mut self, player: dyn Player) {
        self.players.push(player);
    }

    pub fn get_current_player(&self) -> &dyn Player {
        self.players.get(self.current_player_index).unwrap()
    }

    pub fn get_current_player_mut(&mut self) ->  &mut dyn Player {
        self.players.get_mut(self.current_player_index).unwrap()
    }

    pub fn get_next_player_at_mut(&mut self, position: usize) -> &mut dyn Player {
        let index = (self.current_player_index + position) % self.players.len();
        self.players.get_mut(index).unwrap()
    }

    pub fn get_next_player_at(&self, position: usize) -> &dyn Player {
        let index = (self.current_player_index + position) % self.players.len();
        self.players.get(index).unwrap()
    }

    pub fn next_round(&mut self) {
        self.current_player_index = (self.current_player_index + 1) % self.players.len();
    }
}

#[cfg(test)]
mod tests {
    use crate::player::Player;
    use crate::players::Players;

    #[test]
    fn add_players() {
        let mut players = Players::new();
        players.add_player(Player::new("Piet"));
        players.add_player(Player::new("Jan"));
        players.add_player(Player::new("Klaas"));

        assert_eq!(players.players.len(), 3);
    }

    #[test]
    fn get_current_player() {
        let mut players = Players::new();
        players.add_player(Player::new("Piet"));
        players.add_player(Player::new("Jan"));
        players.add_player(Player::new("Klaas"));

        assert_eq!(players.get_current_player().get_name(), "Piet");
        players.next_round();
        assert_eq!(players.get_current_player().get_name(), "Jan");
        players.next_round();
        assert_eq!(players.get_current_player().get_name(), "Klaas");
    }

    #[test]
    fn go_round() {
        let mut players = Players::new();
        players.add_player(Player::new("Piet"));
        players.add_player(Player::new("Jan"));

        assert_eq!(players.get_current_player().get_name(), "Piet");
        players.next_round();
        assert_eq!(players.get_current_player().get_name(), "Jan");
        players.next_round();
        assert_eq!(players.get_current_player().get_name(), "Piet");
    }

    #[test]
    fn get_next_player() {
        let mut players = Players::new();
        players.add_player(Player::new("Piet"));
        players.add_player(Player::new("Jan"));
        players.add_player(Player::new("Klaas"));

        assert_eq!(players.get_next_player_at(0).get_name(), "Piet");
        assert_eq!(players.get_next_player_at(1).get_name(), "Jan");
        assert_eq!(players.get_next_player_at(2).get_name(), "Klaas");
        assert_eq!(players.get_next_player_at(3).get_name(), "Piet");
    }
}