use strum_macros::EnumIter;

pub struct Card {
    pub level: i8,
    pub color: Color
}

#[derive(Clone, Copy, Debug, EnumIter)]
pub enum Color {
    Diamond,
    Club,
    Heart,
    Spade
}