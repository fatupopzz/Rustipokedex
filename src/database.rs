use std::error::Error;
use crate::pokemon::{Pokemon, PokemonType, Stats};
use std::collections::HashMap;

pub struct Database {
    pokemon_map: HashMap<String, Pokemon>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            pokemon_map: HashMap::new(),
        }
    }

    pub fn load_from_csv(&mut self) -> Result<(), Box<dyn Error>> {
        // Lee el CSV incluido en el binario
        let csv_content = include_str!("../assets/pokedex.csv");
        let mut rdr = csv::ReaderBuilder::new()
            .trim(csv::Trim::All)
            .from_reader(csv_content.as_bytes());

        for result in rdr.records() {
            let record = result?;
            
            // Parsear los tipos
            let mut types = Vec::new();
            if let Some(type1) = self.parse_type(&record[2]) {
                types.push(type1);
            }
            if record.get(3).map_or(false, |t| !t.is_empty()) {
                if let Some(type2) = self.parse_type(&record[3]) {
                    types.push(type2);
                }
            }

            let pokemon = Pokemon {
                id: record[0].parse()?,
                name: record[1].to_string(),
                types,
                height: record[4].parse()?,
                weight: record[5].parse()?,
                stats: Stats {
                    hp: record[6].parse()?,
                    attack: record[7].parse()?,
                    defense: record[8].parse()?,
                    special_attack: record[9].parse()?,
                    special_defense: record[10].parse()?,
                    speed: record[11].parse()?,
                },
            };

            self.pokemon_map.insert(pokemon.name.to_lowercase(), pokemon);
        }

        println!("Cargados {} Pokémon desde CSV", self.pokemon_map.len());
        Ok(())
    }

    fn parse_type(&self, type_str: &str) -> Option<PokemonType> {
        match type_str.trim().to_lowercase().as_str() {
            "normal" => Some(PokemonType::Normal),
            "fire" => Some(PokemonType::Fire),
            "water" => Some(PokemonType::Water),
            "electric" => Some(PokemonType::Electric),
            "grass" => Some(PokemonType::Grass),
            "ice" => Some(PokemonType::Ice),
            "fighting" => Some(PokemonType::Fighting),
            "poison" => Some(PokemonType::Poison),
            "ground" => Some(PokemonType::Ground),
            "flying" => Some(PokemonType::Flying),
            "psychic" => Some(PokemonType::Psychic),
            "bug" => Some(PokemonType::Bug),
            "rock" => Some(PokemonType::Rock),
            "ghost" => Some(PokemonType::Ghost),
            "dragon" => Some(PokemonType::Dragon),
            "dark" => Some(PokemonType::Dark),
            "steel" => Some(PokemonType::Steel),
            "fairy" => Some(PokemonType::Fairy),
            _ => None,
        }
    }

    pub fn get_pokemon(&self, id: u16) -> Option<&Pokemon> {
        self.pokemon_map.values().find(|p| p.id == id)
    }

    pub fn get_all_pokemon(&self) -> Vec<&Pokemon> {
        let mut pokemons = self.pokemon_map.values().collect::<Vec<_>>();
        pokemons.sort_by_key(|p| p.id);
        pokemons
    }
}