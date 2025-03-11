use std::collections::HashMap;

pub struct Players {
    pub players: HashMap<u32, Player>,
}
#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub id: u32,
    pub helium: u32,
    pub lithium: u32,
    pub titanium: u32,
    pub owned_planets: Vec<u32>,
    tech_level_fleet: u8,
    tech_level_defense: u8,
    tech_level_economy: u8,
    bot_controlled: bool,
}

impl Players {
    pub fn new() -> Players {
        Players {
            players: HashMap::new(),
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.insert(player.id, player);
    }
}

pub fn generate_player(name: String, id: u32, starter_planet: u32, bot: bool) -> Player {
    let mut p = Player {
        name,
        id,
        helium: 100,
        lithium: 100,
        titanium: 100,
        owned_planets: Vec::new(),
        tech_level_fleet: 0,
        tech_level_defense: 0,
        tech_level_economy: 0,
        bot_controlled: bot,
    };

    p.owned_planets.push(starter_planet);
    return p;
}
