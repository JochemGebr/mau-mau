use maumau::{game::Game, mau_mau::MauMau, player::Player};

pub fn main() {
    let mut game = MauMau::new();
    game.add_player(Player::new("Alice"));
    game.add_player(Player::new("Bob"));
    game.start();
}
