use game::player;
use rand::Rng;
use std::sync::Arc;
use std::sync::Mutex;
use serde::Deserialize;
use std::fs::File;
use std::time::Duration;
use tokio::time::interval;
mod game;

#[derive(Deserialize)]
struct PlanetConfig {
    name: String,
    distance: f64,
    size: u16,
}

#[tokio::main]
async fn main() {
    let file = File::open("./config/config.json").expect("File not found");
    let planets: Vec<PlanetConfig> =
        serde_json::from_reader(file).expect("Error while reading file");

    let mut system = game::planets::System::new("Sol".to_string());

    for (i, planet) in planets.iter().enumerate() {
        let planet = game::planets::generate_planets(planet, i as u32);
        system.add_planet(planet);
    }

    let mut assigned_planets: Vec<usize> = Vec::new();
    let mut rng_gen = rand::rng();

    let mut player_map = player::Players::new();

    for mut n in 0..4 {
        let i = rng_gen.random_range(..system.planets.len() - 1);
        if assigned_planets.contains(&i) {
            n -= 1;
            continue;
        }

        assigned_planets.push(i);

        let mut player = game::player::generate_player("Player".to_string(), n as u32, i as u32, true);
        system.planets[i].owner_id = player.id as i32;
        system.planets[i].owner_name = player.name.clone();
        assigned_planets.push(i);
        player_map.add_player(player);
    }

    let system = Arc::new(Mutex::new(system));
    let player_map = Arc::new(Mutex::new(player_map));


    let mut interval = interval(Duration::from_secs(20));



    loop {
        interval.tick().await;
        system.lock().unwrap().assign_resources(player_map.clone());

        for (key, value) in player_map.lock().unwrap().players.iter() {
            println!("Player: {} Helium: {} Lithium: {} Titanium: {}", value.name, value.helium, value.lithium, value.titanium);
        }

    }


   
}
