use crate::structs::{Clearing, Map};
use strum::IntoEnumIterator; // 0.17.1
use strum_macros::{Display, EnumIter};

#[derive(Default)]
pub struct Decree {
    recruit: Vec<Cards>,
    moveact: Vec<Cards>,
    battle: Vec<Cards>,
    build: Vec<Cards>,
}

#[derive(Default)]
pub struct Birds {
    leader: BirdsLeaders,
    decree: Decree,
    isturn: bool,
    hand: Vec<Cards>,
    turn_num: usize,
    position: Vec<Clearing>,
}

pub struct Charismatic {
    vivizers: Vec<usize>,
    ability: String,
}
pub struct Commander {
    vivizers: Vec<usize>,
    ability: String,
}
pub struct Builder {
    vivizers: Vec<usize>,
    ability: String,
}

pub struct Despot {
    vivizers: Vec<usize>,
    ability: String,
}

#[derive(Debug, EnumIter, Display, Clone, PartialEq, Eq, Default)]
pub enum BirdsLeaders {
    Commander,
    Charismatic,
    Despot,
    #[default]
    Builder,
}
use inquire::Select;

use crate::structs::Cards;
impl Birds {
    pub fn setLeader(&mut self) {
        let mut leader_vec = vec![];
        for leader in BirdsLeaders::iter() {
            leader_vec.push(leader);
        }
        let leader = Select::new("which Leader", leader_vec).prompt();
        self.leader = leader.unwrap();
    }
    pub fn add_to_decree(
        &mut self,
        mut recruit: Option<Vec<Cards>>,
        mut moveact: Option<Vec<Cards>>,
        mut battle: Option<Vec<Cards>>,
        mut builds: Option<Vec<Cards>>,
    ) {
        let mut decree_added_count = 0;
        while decree_added_count < 2 {
            if let Some(ref mut recruit) = recruit {
                self.decree.recruit.append(recruit);
                decree_added_count += recruit.len();
            }

            if let Some(ref mut moveact) = moveact {
                self.decree.moveact.append(moveact);
                decree_added_count += moveact.len();
            }

            if let Some(ref mut battle) = battle {
                self.decree.battle.append(battle);
                decree_added_count += battle.len();
            }
            if let Some(ref mut builds) = battle {
                self.decree.battle.append(builds);
                decree_added_count += builds.len();
            }
        }
    }
    pub fn Default() -> Self {
        Birds {
            leader: BirdsLeaders::Builder,
            decree: {
                Decree {
                    recruit: vec![],
                    moveact: vec![],
                    battle: vec![],
                    build: vec![],
                }
            },
            hand: vec![],
            isturn: false,
            position: vec![],
            turn_num: 0,
        }
    }
    pub fn set_position(&mut self, map: Map) {}
}
