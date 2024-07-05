use crate::game::{Game, GameWithStock};
use crate::pile::Pile;
use crate::players::Players;
use crate::settings::Settings;
use crate::player::{Player, HandedPlayer};

pub struct MauMau {
    settings: Settings,
    players: Players,

    stock: Pile,
    played: Pile,
}

impl MauMau {
    pub fn new() -> MauMau {
        MauMau {
            settings: Settings::new(),
            players: Players::new(),

            stock: Pile::with_deck(),
            played: Pile::new(),
        }
    }
}

impl Game for MauMau {
    fn get_players(&self) -> &Players {
        &self.players
    }

    fn get_players_mut(&mut self) -> &mut Players {
        &mut self.players
    }

    fn get_settings(&self) -> &Settings {
        &self.settings
    }

    fn start(&mut self) {
        self.stock.add_deck();
        self.players.deal(&mut self.stock, 5);

        loop {
            let current_player = self.players.get_current_player_mut();
            let card = current_player.select_card();

            if card.is_some() {
                self.played.add_card(card.unwrap());
            } else {
                break;
            }

            self.players.next_round();
        }
    }
}

impl GameWithStock for MauMau {
    fn get_stock_mut(&mut self) -> &mut Pile {
        &mut self.stock
    }
}


pub struct MauMauPlayer {
    name: String,
    points: u32,
    hand: Pile,
}

impl MauMauPlayer {
    fn add_points(&mut self, points: u32) {
        self.points += points;
    }

    fn get_hand(&self) -> &Pile {
        &self.hand
    }

    fn get_hand_mut(&mut self) -> &mut Pile {
        &mut self.hand
    }

    fn get_name(&self) -> &String {
        &self.name
    }
}

impl Player for MauMauPlayer {
    fn new(name: &str) -> MauMauPlayer {
        MauMauPlayer {
            name: name.to_string(),
            points: 0,
            hand: Pile::new(),
        }
    }
}

impl HandedPlayer for MauMauPlayer {
    fn get_hand(&self) -> &Pile {
        &self.hand
    }

    fn get_hand_mut(&mut self) -> &mut Pile {
        &mut self.hand
    }
}