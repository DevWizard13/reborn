//NOTE: this file is for testing purpouses ONLY, delete it later

use std::mem::discriminant;

use reborn::*;

fn main() {
    let mut world: Vec<Box<dyn Entity>> = vec![Box::new(Player {
        comp: PlayerComp {
            state: PlayerStates::Idle,
        },
    })];
    let mut systems: Vec<Box<dyn System>> = vec![Box::new(PlayerLogic { speed: 0 as i64 })];

    reborn::run(&mut world, &systems);
}

#[derive(Clone)]
enum PlayerStates {
    Walking(f64),
    Idle,
    Jumping,
}

struct Player {
    comp: PlayerComp,
}

#[derive(Clone)]
struct PlayerComp {
    state: PlayerStates,
}

impl reborn::Component for PlayerComp {
    fn get_type(&self) -> &str {
        "PlayerComp"
    }
    fn get_state(&self) -> &str {
        match self.state {
            PlayerStates::Walking(_) => "Walking",
            PlayerStates::Idle => "Idle",
            PlayerStates::Jumping => "Jumping",
        }
    }
    fn change_state(&mut self, state: &str) {
        match state {
            "Walking" => self.state = PlayerStates::Walking(1.0),
            "Idle" => self.state = PlayerStates::Idle,
            "Jumping" => self.state = PlayerStates::Jumping,
            _ => {
                println!("WARNING: ${state} is not valid")
            }
        }
    }
}

impl reborn::Entity for Player {
    fn get_component_names(&self) -> Vec<&str> {
        vec!["PlayerComp"]
    }
    fn change_component_state(&mut self, comp: &str, val: &str) {
        match comp {
            "PlayerComp" => match val {
                "Walking" => self.comp.state = PlayerStates::Walking(0.0),
                "Idle" => self.comp.state = PlayerStates::Idle,
                "Jumping" => self.comp.state = PlayerStates::Jumping,
                _ => println!("WARNING: ${val} is not a valid state"),
            },
            _ => println!("WARNING: ${comp} is not a valid component"),
        }
    }
    fn get_component_val(&self, comp: &str) -> Option<f64> {
        match comp {
            "Walking" => match self.comp.state {
                PlayerStates::Walking(speed) => Some(speed),
                _ => None,
            },
            _ => None,
        }
    }
    fn get_component_state(&self, comp: &str) -> Option<&str> {
        match comp {
            "PlayerComp" => Some(self.comp.get_state()),
            _ => {
                println!("WARNING: ${comp} not a valid component");
                None
            }
        }
    }
}

struct PlayerLogic {
    speed: f64,
}

impl reborn::System for PlayerLogic {
    fn components(&self) -> Vec<&str> {
        vec!["PlayerComp"]
    }
    fn operate(&self, item: &mut dyn Entity) {
        let current_state = item.get_component_state("PlayerComp");
        match current_state {
            None => println!("WARNING: no state found"),
            Some(i) => {
                match i {
                    "Idle" => {
                        item.change_component_state("PlayerComp", "Walking");
                        item.change_component_val("Walking", self.speed);
                        println!("I'm idle btw")
                    }
                    "Walking" => {
                        let speed = match item.get_component_val("PlayerComp") {
                            None => -999.99,
                            Some(j) => j,
                        };
                        println!("I'm walking at speed ${speed}");
                        item.change_component_state("PlayerComp", "Jumping");
                    }
                    "Jumping" => {
                        println!("WEEEEEEEEEEeeeeeeeeeee e e e e e  e  e  e  e  e");
                        item.change_component_state("PlayerComp", "Idle");
                    }
                    _ => println!("I discovered a new state! ...oh, wait, that's not good"),
                };
            }
        }
    }
}
