pub struct System {
    name: String,
    planets: Vec<Planet>,
}

pub struct Planet {
    name: String,
    owner_id: i32,
    owner_name: String,
    distance: f64,
    helium_capacity: u16,
    helium_abundance: u8,
    helium_actual: u16,
    lithium_capacity: u16,
    lithium_abundance: u8,
    lithium_actual: u16,
    titanium_capacity: u16,
    titanium_abundance: u8,
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
}
