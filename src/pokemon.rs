use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PokemonType {
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy,
}

// Implementar Display para PokemonType
impl fmt::Display for PokemonType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PokemonType::Normal => write!(f, "Normal"),
            PokemonType::Fire => write!(f, "Fire"),
            PokemonType::Water => write!(f, "Water"),
            PokemonType::Electric => write!(f, "Electric"),
            PokemonType::Grass => write!(f, "Grass"),
            PokemonType::Ice => write!(f, "Ice"),
            PokemonType::Fighting => write!(f, "Fighting"),
            PokemonType::Poison => write!(f, "Poison"),
            PokemonType::Ground => write!(f, "Ground"),
            PokemonType::Flying => write!(f, "Flying"),
            PokemonType::Psychic => write!(f, "Psychic"),
            PokemonType::Bug => write!(f, "Bug"),
            PokemonType::Rock => write!(f, "Rock"),
            PokemonType::Ghost => write!(f, "Ghost"),
            PokemonType::Dragon => write!(f, "Dragon"),
            PokemonType::Dark => write!(f, "Dark"),
            PokemonType::Steel => write!(f, "Steel"),
            PokemonType::Fairy => write!(f, "Fairy"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pokemon {
    pub id: u16,
    pub name: String,
    pub types: Vec<PokemonType>,
    pub height: u16,
    pub weight: u16,
    pub stats: Stats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub hp: u8,
    pub attack: u8,
    pub defense: u8,
    pub special_attack: u8,
    pub special_defense: u8,
    pub speed: u8,
}

impl Pokemon {
    // Método para obtener el tipo primario
    pub fn primary_type(&self) -> Option<&PokemonType> {
        self.types.first()
    }

    // Método para obtener el tipo secundario
    pub fn secondary_type(&self) -> Option<&PokemonType> {
        if self.types.len() > 1 {
            self.types.get(1)
        } else {
            None
        }
    }

    // Método para obtener el total de stats
    pub fn total_stats(&self) -> u16 {
        self.stats.hp as u16 +
        self.stats.attack as u16 +
        self.stats.defense as u16 +
        self.stats.special_attack as u16 +
        self.stats.special_defense as u16 +
        self.stats.speed as u16
    }

    // Método para formatear altura y peso
    pub fn formatted_height(&self) -> f32 {
        self.height as f32 / 10.0
    }

    pub fn formatted_weight(&self) -> f32 {
        self.weight as f32 / 10.0
    }
}

impl Stats {
    // Método para obtener el stat más alto
    pub fn highest_stat(&self) -> (&str, u8) {
        let stats = [
            ("HP", self.hp),
            ("Ataque", self.attack),
            ("Defensa", self.defense),
            ("Atq. Esp.", self.special_attack),
            ("Def. Esp.", self.special_defense),
            ("Velocidad", self.speed),
        ];
        
        stats.iter()
            .max_by_key(|(_, value)| value)
            .map(|&(name, value)| (name, value))
            .unwrap_or(("HP", 0))
    }

    // Método para obtener el stat más bajo
    pub fn lowest_stat(&self) -> (&str, u8) {
        let stats = [
            ("HP", self.hp),
            ("Ataque", self.attack),
            ("Defensa", self.defense),
            ("Atq. Esp.", self.special_attack),
            ("Def. Esp.", self.special_defense),
            ("Velocidad", self.speed),
        ];
        
        stats.iter()
            .min_by_key(|(_, value)| value)
            .map(|&(name, value)| (name, value))
            .unwrap_or(("HP", 0))
    }

    // Método para obtener el promedio de stats
    pub fn average_stat(&self) -> f32 {
        (self.hp as f32 +
         self.attack as f32 +
         self.defense as f32 +
         self.special_attack as f32 +
         self.special_defense as f32 +
         self.speed as f32) / 6.0
    }
}

// Implementar PartialOrd para Pokemon para poder ordenarlos
impl PartialOrd for Pokemon {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.id.cmp(&other.id))
    }
}

impl PartialEq for Pokemon {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Ord for Pokemon {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl Eq for Pokemon {}