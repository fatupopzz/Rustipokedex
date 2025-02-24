mod pokemon;
mod database;
mod ui;

use database::Database;
use ui::PokedexApp;
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    println!("Iniciando PokéDex...");
    
    // Inicializar la base de datos desde el CSV
    let mut database = Database::new();
    match database.load_from_csv() {
        Ok(_) => println!("Base de datos cargada exitosamente"),
        Err(e) => {
            eprintln!("Error al cargar la base de datos: {}", e);
            return Ok(());
        }
    }

    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(900.0, 600.0)),
        min_window_size: Some(egui::vec2(800.0, 500.0)),
        centered: true,
        transparent: true,
        default_theme: eframe::Theme::Light,
        ..Default::default()
    };

    eframe::run_native(
        "PokéDex pero en Rust",
        native_options,
        Box::new(|_cc| Box::new(PokedexApp::new(database)))
    )
}