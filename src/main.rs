// #![allow(dead_code)]

use std::env;
use std::process::exit;

mod battle;
mod constants;

use battle::{entity::Entity, Battle};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Need enemy file config!");
        exit(1);
    }

    let mut battle = Battle::new();
    let mut entities: Vec<Entity> = Vec::new();
    battle.load_players(&mut entities);
    battle.load_enemies(&mut entities, &args[1]);

    battle.sort(entities);
    battle.main_loop();
}
