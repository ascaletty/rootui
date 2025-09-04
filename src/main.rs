use rootui::structs::*;
mod setup;

use strum::IntoEnumIterator;

use inquire::{Select, Text};
fn main() {
    let i = 0;
    println!("Please roll for player pick order");
    let num_players: usize = Text::new("Number of players")
        .prompt()
        .unwrap()
        .parse()
        .unwrap();
    if num_players > 4 {
        panic!("bruh i can only do 4 players rn");
    }

    let mut factions = vec![];
    for faction in rootui::structs::FactionType::iter() {
        factions.push(faction);
    }
    setup::setup(num_players, &mut factions);
    rootui::maps::mapgen::ui();
}
