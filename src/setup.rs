use inquire::{Select, Text};
use rootui::birds_structs::Birds;
use rootui::structs::*;

pub fn setup(num_players: usize, factions: &mut Vec<FactionType>) -> Vec<Player> {
    let mut player_vec = Vec::new();

    for player_num in 1..=num_players {
        let name: String = Text::new("what is your name?").prompt().unwrap();

        let faction = Select::new("pick your faction", factions.clone())
            .prompt()
            .unwrap();
        // Find the index of the chosen faction and remove it
        if let Some(pos) = factions.iter().position(|f| *f == faction) {
            factions.remove(pos);
        }

        player_vec.push(Player {
            name,
            faction,
            turn_num: player_num,
        });
    }
    let mut birds = Birds::default();
    Birds::setLeader(&mut birds);
    player_vec
}
