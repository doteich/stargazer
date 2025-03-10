pub struct Players{
    players: Map<i32, Player>,
}


pub struct Player {
    name: String,
    id: u32,
    helium: u16,
    lithium: u16,
    titanium: u16,
    owned_planets: Vec<u32>,
    tech_level_fleet: u8,
    tech_level_defense: u8,
    tech_level_economy: u8,
    bot_controlled: bool,
}

impl Players {
    pub fn new() -> Players {
        Players {
            players: Map::new(),
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.insert(player.id, player);
    }
}
