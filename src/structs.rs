use strum::IntoEnumIterator; // 0.17.1
use strum_macros::{Display, EnumIter}; // 0.17.1
pub enum CardSuit {
    Fox,
    Mouse,
    Bunny,
    Bird,
}
struct Ambush {
    cardtype: CardSuit,
}

struct Dominance {
    cardtype: CardSuit,
}

#[derive(Debug, EnumIter, Display, Clone, PartialEq, Eq)]
pub enum Cards {
    Ambush,
    Dominance,
}
pub struct Card {
    cardtype: CardSuit,
    card: Cards,
}
#[derive(Debug, EnumIter, Display, Clone, PartialEq, Eq)]
pub enum FactionType {
    Birds,
    WA,
    Vagabond,
    Cats,
}

pub struct Player {
    pub faction: FactionType,
    pub turn_num: usize,
    pub name: String,
}
pub enum ClearingType {
    Fox,
    Bunny,
    Rabbit,
    None,
}
pub enum MapTypes {
    Winter,
    Mountain,
    Lake,
    Forest,
}
pub enum Animals {
    Mouse(usize),
    Cat(usize),
    Rouge(usize),
    Birds(usize),
}
pub enum Tokens {
    Simps(usize),
    Mobs(usize),
    Wood(usize),
}
pub enum Buildings {
    Roosts(usize),
    Ruins(usize),
    Sawmills(usize),
    Anvils(usize),
    Recruiters(usize),
}

pub struct Clearing {
    pub suit: ClearingType,
    pub inhabitants: Vec<Animals>,
    pub adjacent: Vec<usize>,
    pub river_adj: bool,
    pub id: usize,
    pub token: Vec<Tokens>,
    pub buildings: Vec<Buildings>,
    pub build_slots: usize,
    pub ruins: bool,
}
pub enum Focus {
    Left,
    Right,
}
pub struct Map {
    pub board: MapTypes,
    pub clearings: Vec<Clearing>,
    pub exit: bool,
    pub focus: Focus,
}
