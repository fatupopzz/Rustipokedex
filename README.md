## Una pokedex en Rust pero con una interfaz en egui

<img width="840" alt="image" src="https://github.com/user-attachments/assets/ea3263ff-690e-4fd8-8dad-c4859f589cc3" />

La verdad que experiencia mas horrible 

## Estructura del proyecto

- src/main.rs - Punto de entrada de la aplicación
- src/pokemon.rs - Definiciones de las estructuras y métodos para los Pokémon
- src/database.rs - Lógica para cargar y gestionar los datos desde el CSV
- src/ui/mod.rs - Implementación de la interfaz gráfica con egui
- assets/pokedex.csv - Base de datos de Pokémon en formato CSV

## Tecnologías utilizadas

- Rust - Lenguaje de programación seguro y de alto rendimiento
- egui - Framework ligero para interfaces gráficas en Rust
- serde - Serialización/deserialización de datos
- csv - Manipulación de archivos CSV

## como correr

- Cargo build
- Cargo run
