use crate::database::Database;
use crate::pokemon::PokemonType;  
use eframe::egui;

pub struct PokedexApp {
    database: Database,
    selected_pokemon: Option<u16>,
    search_term: String,
}

impl PokedexApp {
    pub fn new(database: Database) -> Self {
        Self {
            database,
            selected_pokemon: None,
            search_term: String::new(),
        }
    }
}

impl eframe::App for PokedexApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();
        style.visuals.window_fill = egui::Color32::from_rgba_premultiplied(240, 255, 245, 180);
        style.visuals.panel_fill = egui::Color32::from_rgba_premultiplied(255, 255, 255, 180);
        style.visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgba_premultiplied(255, 255, 255, 230);
        ctx.set_style(style);

        egui::CentralPanel::default()
            .frame(egui::Frame::none()
                .fill(egui::Color32::from_rgba_premultiplied(240, 255, 245, 130))
                .shadow(egui::epaint::Shadow {
                    extrusion: 8.0,
                    color: egui::Color32::from_rgba_premultiplied(0, 0, 0, 60),
                }))
            .show(ctx, |ui| {
                // Header
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    ui.heading(
                        egui::RichText::new("PokéDex")
                            .size(40.0)
                            .color(egui::Color32::from_rgb(51, 153, 102))
                            .strong()
                    );
                    ui.add_space(5.0);
                    ui.label(
                        egui::RichText::new("GRAAWW POKEDEX EN RUST")
                            .size(18.0)
                            .color(egui::Color32::from_rgb(100, 180, 140))
                            .strong()
                    );
                    ui.add_space(20.0);
                });

                // Search bar
                render_search_bar(ui, &mut self.search_term);
                ui.add_space(20.0);

                // Main content - Split view
                ui.horizontal(|ui| {
                    // Left panel - Pokemon List
                    render_pokemon_list_panel(ui, &self.database, &self.search_term, &mut self.selected_pokemon);
                    
                    ui.add_space(20.0);
                    
                    // Right panel - Pokemon Details
                    render_pokemon_details_panel(ui, &self.database, self.selected_pokemon);
                });
            });
    }
}

fn render_search_bar(ui: &mut egui::Ui, search_term: &mut String) {
    ui.vertical_centered(|ui| {
        egui::Frame::none()
            .fill(egui::Color32::from_rgba_premultiplied(255, 255, 255, 200))
            .rounding(12.0)
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(180, 220, 200)))
            .shadow(egui::epaint::Shadow {
                extrusion: 5.0,
                color: egui::Color32::from_rgba_premultiplied(0, 0, 0, 40),
            })
            .show(ui, |ui| {
                ui.set_width(ui.available_width() * 0.7);
                ui.horizontal(|ui| {
                    ui.label("🔍");
                    ui.add_sized(
                        [ui.available_width() - 30.0, 24.0],
                        egui::TextEdit::singleline(search_term)
                            .hint_text("Buscar Pokémon...")
                            .text_color(egui::Color32::from_rgb(60, 100, 80))
                    );
                });
            });
    });
}

fn render_pokemon_list_panel(
    ui: &mut egui::Ui,
    database: &Database,
    search_term: &str,
    selected_pokemon: &mut Option<u16>,
) {
    egui::Frame::none()
        .fill(egui::Color32::from_rgba_premultiplied(255, 255, 255, 200))
        .rounding(12.0)
        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(180, 220, 200)))
        .shadow(egui::epaint::Shadow {
            extrusion: 8.0,
            color: egui::Color32::from_rgba_premultiplied(0, 0, 0, 40),
        })
        .show(ui, |ui| {
            ui.set_width(400.0);
            ui.set_height(400.0);
            ui.vertical(|ui| {
                ui.heading(
                    egui::RichText::new("Pokémon")
                        .size(24.0)
                        .color(egui::Color32::from_rgb(51, 153, 102))
                );
                ui.add_space(8.0);
                
                egui::ScrollArea::vertical()
                    .max_height(350.0)
                    .show(ui, |ui| {
                        for pokemon in database.get_all_pokemon() {
                            if pokemon.name.to_lowercase().contains(&search_term.to_lowercase()) {
                                let is_selected = Some(pokemon.id) == *selected_pokemon;
                                let button = ui.add_sized(
                                    [ui.available_width(), 36.0],
                                    egui::Button::new(
                                        egui::RichText::new(format!("#{:03} - {}", pokemon.id, pokemon.name))
                                            .color(if is_selected {
                                                egui::Color32::from_rgb(51, 153, 102)
                                            } else {
                                                egui::Color32::from_rgb(80, 120, 100)
                                            })
                                    )
                                    .fill(if is_selected {
                                        egui::Color32::from_rgba_premultiplied(173, 216, 230, 255)
                                    } else {
                                        egui::Color32::TRANSPARENT
                                    })
                                );
                                
                                if button.clicked() {
                                    *selected_pokemon = Some(pokemon.id);
                                }
                                ui.add_space(4.0);
                            }
                        }
                    });
            });
        });
}

fn render_pokemon_details_panel(
    ui: &mut egui::Ui,
    database: &Database,
    selected_pokemon: Option<u16>,
) {
    // Panel principal de detalles
    egui::Frame::none()
        .fill(egui::Color32::from_rgba_premultiplied(255, 255, 255, 200))
        .rounding(12.0)
        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(180, 220, 200)))
        .shadow(egui::epaint::Shadow {
            extrusion: 8.0,
            color: egui::Color32::from_rgba_premultiplied(0, 0, 0, 40),
        })
        .show(ui, |ui| {
            ui.set_width(400.0);
            ui.set_height(400.0);
            
            if let Some(id) = selected_pokemon {
                if let Some(pokemon) = database.get_pokemon(id) {
                    // Usamos un ScrollArea para que todo sea visible si hay mucho contenido
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        // Nombre y número del Pokémon
                        ui.vertical_centered(|ui| {
                            ui.heading(
                                egui::RichText::new(&pokemon.name)
                                    .size(32.0)
                                    .color(egui::Color32::from_rgb(51, 153, 102))
                            );
                            ui.label(
                                egui::RichText::new(format!("#{:03}", pokemon.id))
                                    .size(18.0)
                                    .color(egui::Color32::from_rgb(100, 180, 140))
                            );
                        });
                        
                        ui.add_space(15.0);
                        
                        // Sección de Información Básica
                        ui.push_id("basic_info", |ui| {
                            egui::Frame::none()
                                .fill(egui::Color32::from_rgba_premultiplied(240, 255, 245, 200))
                                .rounding(8.0)
                                .shadow(egui::epaint::Shadow {
                                    extrusion: 4.0,
                                    color: egui::Color32::from_rgba_premultiplied(0, 0, 0, 30),
                                })
                                .show(ui, |ui| {
                                    ui.heading(
                                        egui::RichText::new("Información Básica")
                                            .size(18.0)
                                            .color(egui::Color32::from_rgb(51, 153, 102))
                                    );
                                    ui.add_space(8.0);
                                    
                                    // Altura y Peso
                                    ui.horizontal(|ui| {
                                        ui.label(format!("Altura: {:.1} m", pokemon.height as f32 / 10.0));
                                        ui.add_space(15.0);
                                        ui.label(format!("Peso: {:.1} kg", pokemon.weight as f32 / 10.0));
                                    });
                                    
                                    // Tipos
                                    ui.add_space(8.0);
                                    ui.label("Tipos:");
                                    ui.horizontal(|ui| {
                                        for type_info in &pokemon.types {
                                            render_type_badge(ui, type_info);
                                            ui.add_space(5.0);
                                        }
                                    });
                                });
                        });
                        
                        ui.add_space(15.0);
                        
                        // Sección de Estadísticas
                        ui.push_id("stats", |ui| {
                            egui::Frame::none()
                                .fill(egui::Color32::from_rgba_premultiplied(240, 255, 245, 200))
                                .rounding(8.0)
                                .shadow(egui::epaint::Shadow {
                                    extrusion: 4.0,
                                    color: egui::Color32::from_rgba_premultiplied(0, 0, 0, 30),
                                })
                                .show(ui, |ui| {
                                    ui.heading(
                                        egui::RichText::new("Estadísticas")
                                            .size(18.0)
                                            .color(egui::Color32::from_rgb(51, 153, 102))
                                    );
                                    ui.add_space(8.0);
                                    
                                    // Barras de estadísticas
                                    render_stat_bar(ui, "HP", pokemon.stats.hp);
                                    render_stat_bar(ui, "Ataque", pokemon.stats.attack);
                                    render_stat_bar(ui, "Defensa", pokemon.stats.defense);
                                    render_stat_bar(ui, "Atq. Esp.", pokemon.stats.special_attack);
                                    render_stat_bar(ui, "Def. Esp.", pokemon.stats.special_defense);
                                    render_stat_bar(ui, "Velocidad", pokemon.stats.speed);
                                });
                        });
                    });
                }
            } else {
                ui.vertical_centered(|ui| {
                    ui.add_space(100.0);
                    ui.label(
                        egui::RichText::new("Selecciona un Pokémon")
                            .size(24.0)
                            .color(egui::Color32::from_rgb(100, 180, 140))
                    );
                });
            }
        });
}

fn render_type_badge(ui: &mut egui::Ui, pokemon_type: &PokemonType) {
    // Obtener el color según el tipo de Pokémon
    let type_color = match pokemon_type {
        PokemonType::Normal => egui::Color32::from_rgb(168, 168, 120),
        PokemonType::Fire => egui::Color32::from_rgb(240, 128, 48),
        PokemonType::Water => egui::Color32::from_rgb(104, 144, 240),
        PokemonType::Grass => egui::Color32::from_rgb(120, 200, 80),
        PokemonType::Electric => egui::Color32::from_rgb(248, 208, 48),
        PokemonType::Ice => egui::Color32::from_rgb(152, 216, 216),
        PokemonType::Fighting => egui::Color32::from_rgb(192, 48, 40),
        PokemonType::Poison => egui::Color32::from_rgb(160, 64, 160),
        PokemonType::Ground => egui::Color32::from_rgb(224, 192, 104),
        PokemonType::Flying => egui::Color32::from_rgb(168, 144, 240),
        PokemonType::Psychic => egui::Color32::from_rgb(248, 88, 136),
        PokemonType::Bug => egui::Color32::from_rgb(168, 184, 32),
        PokemonType::Rock => egui::Color32::from_rgb(184, 160, 56),
        PokemonType::Ghost => egui::Color32::from_rgb(112, 88, 152),
        PokemonType::Dragon => egui::Color32::from_rgb(112, 56, 248),
        PokemonType::Dark => egui::Color32::from_rgb(112, 88, 72),
        PokemonType::Steel => egui::Color32::from_rgb(184, 184, 208),
        PokemonType::Fairy => egui::Color32::from_rgb(238, 153, 172),
        _ => egui::Color32::from_rgb(180, 180, 180), // Color por defecto
    };

    egui::Frame::none()
        .fill(type_color)
        .rounding(12.0)
        .shadow(egui::epaint::Shadow {
            extrusion: 2.0,
            color: egui::Color32::from_rgba_premultiplied(0, 0, 0, 20),
        })
        .show(ui, |ui| {
            // Usar un contenedor centrado para el texto
            ui.horizontal_centered(|ui| {
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(format!("{:?}", pokemon_type))
                            .color(egui::Color32::WHITE)
                            .strong()
                            .text_style(egui::TextStyle::Small)
                    )
                );
            });
        });
}

fn render_stat_bar(ui: &mut egui::Ui, name: &str, value: u8) {
    ui.horizontal(|ui| {
        ui.set_min_width(80.0);
        ui.label(
            egui::RichText::new(name)
                .color(egui::Color32::from_rgb(60, 100, 80))
        );
        
        let progress = value as f32 / 255.0;
        let bar = egui::ProgressBar::new(progress)
            .text(value.to_string())
            .fill(egui::Color32::from_rgb(
                (200.0 - progress * 100.0) as u8,
                (180.0 + progress * 75.0) as u8,
                (140.0 + progress * 40.0) as u8,
            ));
            
        ui.add(bar);
    });
    ui.add_space(4.0);
}