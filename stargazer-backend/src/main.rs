use axum::Router;
use game::planets;
use game::player;
use rand::Rng;
use serde::Deserialize;
use std::fs::File;
use std::sync::Arc;
use std::sync::Mutex;
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
    let planets = match planets::init_from_file("./config/planets.json") {
        Ok(planets) => planets,
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    };

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

        let mut player =
            game::player::generate_player("Player".to_string(), n as u32, i as u32, true);
        system.planets[i].owner_id = player.id as i32;
        system.planets[i].owner_name = player.name.clone();
        assigned_planets.push(i);
        player_map.add_player(player);
    }

    let system = Arc::new(Mutex::new(system));
    let player_map = Arc::new(Mutex::new(player_map));

    tokio::spawn(system_event_loop(5, system.clone(), player_map.clone()));

    let router = create_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, router).await.unwrap();
}

fn create_router() -> Router {
    Router::new()
}

async fn system_event_loop(
    iv: u64,
    system: Arc<Mutex<game::planets::System>>,
    player_map: Arc<Mutex<player::Players>>,
) {
    let mut interval = interval(Duration::from_secs(iv));

    loop {
        interval.tick().await;
        system.lock().unwrap().assign_resources(player_map.clone());

        for (key, value) in player_map.lock().unwrap().players.iter() {
            println!(
                "Player: {} Helium: {} Lithium: {} Titanium: {}",
                value.name, value.helium, value.lithium, value.titanium
            );
        }
    }
}
