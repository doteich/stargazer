use crate::PlanetConfig;

use rand::Rng;
use std::sync::Arc;
use std::sync::Mutex;

use super::player::Players;
pub struct System {
    pub name: String,
    pub planets: Vec<Planet>,
}

pub struct Planet {
    name: String,
    pub owner_id: i32,
    pub owner_name: String,
    distance: f64,
    helium_capacity: u16,
    helium_abundance: u32,
    helium_actual: u16,
    lithium_capacity: u16,
    lithium_abundance: u32,
    lithium_actual: u16,
    titanium_capacity: u16,
    titanium_abundance: u32,
    titanium_actual: u16,
    planet_id: u32,
}

impl System {
    pub fn new(name: String) -> System {
        System {
            name,
            planets: Vec::new(),
        }
    }

    pub fn add_planet(&mut self, planet: Planet) {
        self.planets.push(planet);
    }
    pub fn assign_resources(&mut self, player_map: Arc<Mutex<Players>>) {
        for planet in self.planets.iter_mut() {
            if planet.owner_id > 0 {
                let mut player_map = player_map.lock().unwrap();
                let player = player_map
                    .players
                    .get_mut(&(planet.owner_id as u32))
                    .unwrap();
                player.helium += planet.helium_abundance * planet.helium_capacity as u32;
                player.lithium += planet.lithium_abundance * planet.lithium_capacity as u32;
                player.titanium += planet.titanium_abundance * planet.titanium_capacity as u32;
            }
        }
    }
}

pub fn generate_planets(conf: &PlanetConfig, id: u32) -> Planet {
    let mut rng_gen = rand::rng();
    let helium_abundance =
        rng_gen.random_range(1 * conf.distance as u32..100 * (conf.distance * 2.50) as u32);
    let titanium_abundance =
        rng_gen.random_range(1 * (100 - conf.distance as u32)..100 * (100 - conf.distance as u32));
    let l_factor = rng_gen.random_range(1..100);
    let lithium_abundance: u32 = rng_gen.random_range(1 * l_factor..100 * l_factor);
    let helium_capacity = rng_gen.random_range(5..conf.size * 5);
    let lithium_capacity = rng_gen.random_range(5..conf.size * 5);
    let titanium_capacity = rng_gen.random_range(5..conf.size * 5);

    Planet {
        name: conf.name.clone(),
        owner_id: -1,
        owner_name: "None".to_string(),
        distance: conf.distance,
        helium_capacity: helium_capacity,
        helium_abundance: helium_abundance,
        helium_actual: 0,
        lithium_capacity: lithium_capacity,
        lithium_abundance: lithium_abundance,
        lithium_actual: 0,
        titanium_capacity: titanium_capacity,
        titanium_abundance: titanium_abundance,
        titanium_actual: 0,
        planet_id: id,
    }
}
