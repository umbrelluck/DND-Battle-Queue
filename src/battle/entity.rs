use serde_derive::Deserialize;
use std::io;

use crate::constants::INACTIVE;

// FIX: Change struct fileds to private and get accessors

#[derive(Debug, Deserialize)]
pub struct Entity {
    name: String,
    pub initiative: i16,
    alive: bool,
    pub ac: i16,
    pub max_hp: i16,
    pub hp: i16,
    damage_taken: Vec<i16>,
    pub dexterity: i16,
    pub wisdom: i16,
    npc: bool,
}

impl Entity {
    pub fn new(name: String, npc: bool) -> Self {
        Entity {
            name,
            initiative: INACTIVE,
            alive: true,
            ac: INACTIVE,
            max_hp: INACTIVE,
            hp: INACTIVE,
            damage_taken: vec![],
            dexterity: INACTIVE,
            wisdom: INACTIVE,
            npc,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_initiative(&self) -> i16 {
        self.initiative
    }

    pub fn get_dexterity(&self) -> i16 {
        self.dexterity
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn is_npc(&self) -> bool {
        self.npc
    }

    pub fn mark_dead(&mut self) {
        self.alive = false;
    }

    pub fn take_damage(&mut self, ammount: i16) {
        self.hp -= ammount;
        self.damage_taken.push(ammount);
        if self.hp < 0 && self.max_hp > 0 {
            self.mark_dead();
        }
    }

    pub fn print_info(&self) {
        println!(
            "Character information:\
            \n\tName: {},\
            \n\tInitiative: {},\
            \n\tAlive: {},\
            \n\tAC: {},\
            \n\tMax HP: {},\
            \n\tHP: {},\
            \n\tDexterity: {},\
            \n\tWisdom: {},\
            \n\tDamage Taken: {:?}",
            self.name,
            self.initiative,
            self.alive,
            self.ac,
            self.max_hp,
            self.hp,
            self.dexterity,
            self.wisdom,
            self.damage_taken
        )
    }

    pub fn get_dexterity_input(&mut self) {
        if self.dexterity == INACTIVE {
            println!("Enter dexterity for {}", self.name);
            let mut buf = String::new();
            io::stdin()
                .read_line(&mut buf)
                .expect("Failed to read input");
            let num: i16 = buf.trim().parse().expect("Not a number");
            self.dexterity = num;
        }
    }
}
