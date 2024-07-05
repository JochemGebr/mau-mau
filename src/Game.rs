use crate::pile::Pile;
use crate::player::Player;
use crate::players::Players;
use crate::settings::Settings;

pub trait GameWithStock: Game {
    fn get_stock_mut(&mut self) -> &mut Pile;
}

pub trait Game {
    fn start(&mut self);

    fn get_players(&self) -> &Players;
    fn get_players_mut(&mut self) -> &mut Players;
    fn get_settings(&self) -> &Settings;

    fn add_player(&mut self, player: dyn Player) {
        self.get_players_mut().add_player(player);
    }
}
