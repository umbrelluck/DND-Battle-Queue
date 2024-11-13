use std::cmp::Reverse;
use std::collections::VecDeque;
use std::fs;
use std::io::{self, Write};
use toml::Table;

// mod crate
pub mod entity;

use crate::constants::{INACTIVE, PLAYERS_CONFIG};
use entity::Entity;

pub struct Battle {
    player_count: i16,
    enemy_count: i16,
    queue: VecDeque<Entity>,
}

impl Battle {
    pub fn new() -> Self {
        Battle {
            player_count: INACTIVE,
            enemy_count: INACTIVE,
            queue: VecDeque::new(),
        }
    }

    pub fn main_loop(&mut self) {
        let mut end_turn: bool;

        while self.player_count > 0 && self.enemy_count > 0 {
            if !self.queue[0].is_alive() {
                let ent = self.queue.pop_front().unwrap();
                self.queue.push_back(ent);
                continue;
            }

            end_turn = false;
            println!("=============================================");
            print!("Current queue   > ");
            self.print_queue();
            print!("Current turn    > ");
            println!("{}", self.queue[0].get_name());
            println!("=============================================");

            println!(
                "Choose your action:\n\t \
                d - deal damage\n\t \
                m - mark dead\n\t \
                i - get information\n\t \
                ii - get informaion about current\n\t \
                v - view this message again\n\t \
                q - view queue again\n\t \
                e - end turn"
            );

            while !end_turn {
                print!("\n  > ");
                io::stdout().flush().expect("flush failed!");
                let mut buf = String::new();
                io::stdin()
                    .read_line(&mut buf)
                    .expect("Failed to read input");
                match buf.trim() {
                    "d" => {
                        let o_target = self.find_target();
                        match o_target {
                            Some(target) => {
                                if target.is_alive() {
                                    println!("Enter damage:");
                                    let damage: i16;

                                    loop {
                                        print!("\n  > ");
                                        io::stdout().flush().expect("flush failed!");
                                        let mut buf = String::new();
                                        io::stdin()
                                            .read_line(&mut buf)
                                            .expect("Failed to read input");

                                        match buf.trim().parse() {
                                            Ok(num) => {
                                                damage = num;
                                                break;
                                            }
                                            Err(_) => println!("Not a number"),
                                        }
                                    }

                                    target.take_damage(damage);

                                    if !target.is_alive() {
                                        if target.is_npc() {
                                            self.enemy_count -= 1;
                                        } else {
                                            self.player_count -= 1;
                                        }
                                    }
                                    println!("Operation completed");
                                } else {
                                    println!("Target is already dead...");
                                }
                            }
                            None => println!("Going back..."),
                        }
                        println!("\nYou are now in action selection menu");
                        println!("------------------------------------");
                    }
                    "m" => {
                        let o_target = self.find_target();
                        match o_target {
                            Some(target) => {
                                if target.is_alive() {
                                    target.mark_dead();
                                    if target.is_npc() {
                                        self.enemy_count -= 1;
                                    } else {
                                        self.player_count -= 1;
                                    }
                                    println!("Operation completed");
                                } else {
                                    println!("Target is already dead...");
                                }
                            }
                            None => println!("Going back..."),
                        }
                        println!("\nYou are now in action selection menu");
                        println!("------------------------------------");
                    }
                    "i" => {
                        let o_target = self.find_target();
                        match o_target {
                            Some(target) => {
                                target.print_info();
                                println!("Operation completed");
                            }
                            None => println!("Going back..."),
                        }
                        println!("\nYou are still in action selection menu");
                        println!("-------------------------------------");
                    }
                    "ii" => self.queue[0].print_info(),
                    "v" => println!(
                        "Choose your action:\n\t \
                        d - deal damage\n\t \
                        m - mark dead\n\t \
                        i - get information\n\t \
                        ii - get informaion about current\n\t \
                        v - view this message again\n\t \
                        q - view queue again\n\t \
                        e - end turn"
                    ),
                    "q" => {
                        print!("Current queue   > ");
                        self.print_queue();
                    }
                    "e" => end_turn = true,
                    _ => println!("Incorrect option"),
                }
            }

            println!("\n---------< End of current turn >---------\n");
            let current_entity = self.queue.pop_front().unwrap();
            self.queue.push_back(current_entity);
        }
    }

    pub fn find_target(&mut self) -> Option<&mut Entity> {
        println!(
            "Select target:\n\t \
                by index\n\t \
                by name\n\
            Or press 'b' to go back"
        );

        let mut target: String = String::new();

        let mut stop_selection: bool = false;
        while !stop_selection {
            print!("\n  > ");
            io::stdout().flush().expect("flush failed!");
            let mut buf = String::new();
            io::stdin()
                .read_line(&mut buf)
                .expect("Failed to read input");

            if buf.trim() == "b" {
                target = String::from("b");
                break;
            }

            let o_result = match buf.trim().parse::<usize>() {
                Ok(num) => self.find_target_by_index(num),
                Err(_) => self.find_target_by_name(buf.trim()),
            };

            if o_result.is_some() {
                stop_selection = true;
                target = String::from(buf.trim());
            } else {
                println!("No such target found");
            }
        }

        match target.trim().parse() {
            Ok(num) => self.find_target_by_index(num),
            Err(_) => self.find_target_by_name(target.trim()),
        }
    }

    fn find_target_by_name(&mut self, name: &str) -> Option<&mut Entity> {
        self.queue
            .iter_mut()
            .find(|e| e.get_name() == name && e.is_alive())
    }

    fn find_target_by_index(&mut self, index: usize) -> Option<&mut Entity> {
        self.queue.get_mut(index)
    }

    pub fn sort(&mut self, mut entities: Vec<Entity>) {
        entities.sort_by_key(|x| Reverse(x.get_initiative()));

        let mut flag: bool = false;

        for i in (1..entities.len()).rev() {
            if entities[i].get_initiative() == entities[i - 1].get_initiative() {
                entities[i].get_dexterity_input();
                entities[i - 1].get_dexterity_input();
                flag |= true;
            }
        }
        if flag {
            entities.sort_by(|a, b| {
                if a.get_initiative() != b.get_initiative() {
                    b.get_initiative().cmp(&a.get_initiative())
                } else {
                    b.get_dexterity().cmp(&a.get_dexterity())
                }
            })
        }
        self.queue = VecDeque::from(entities);
    }

    pub fn load_players(&mut self, entities: &mut Vec<Entity>) {
        self.player_count = Battle::load_entities_from_toml(entities, PLAYERS_CONFIG, false);
    }

    pub fn load_enemies(&mut self, entities: &mut Vec<Entity>, filename: &str) {
        self.enemy_count = Battle::load_entities_from_toml(entities, filename, true);
    }

    fn load_entities_from_toml(entities: &mut Vec<Entity>, filename: &str, npc: bool) -> i16 {
        let content = fs::read_to_string(filename).unwrap();
        let main_table = content.parse::<Table>().unwrap();

        for (name, table) in main_table.iter() {
            let mut entity = Entity::new(String::from(name), npc);

            if table.get("initiative").is_some() {
                entity.initiative = table["initiative"].as_integer().unwrap() as i16;
            }

            if table.get("ac").is_some() {
                entity.ac = table["ac"].as_integer().unwrap() as i16;
            }

            if table.get("hp").is_some() {
                entity.max_hp = table["hp"].as_integer().unwrap() as i16;
                entity.hp = entity.max_hp;
            }

            if table.get("dexterity").is_some() {
                entity.dexterity = table["dexterity"].as_integer().unwrap() as i16;
            }

            if table.get("wisdom").is_some() {
                entity.wisdom = table["wisdom"].as_integer().unwrap() as i16;
            }

            entities.push(entity);
        }

        main_table.len() as i16
    }

    fn print_queue(&self) {
        let names_with_indices: Vec<String> = self
            .queue
            .iter()
            .enumerate()
            .filter_map(|(i, e)| {
                if e.is_alive() {
                    Some(format!("{i}:{}", e.get_name()))
                } else {
                    None
                }
            })
            .collect();

        for (i, chunk) in names_with_indices.chunks(7).enumerate() {
            let line = chunk.join(" -> ");

            if i == 0 {
                println!("{}", line); // First line without indentation
            } else {
                println!("{:>14}-> {}", " ", line); // Subsequent lines with 15 leading spaces
            }
        }
    }
}
