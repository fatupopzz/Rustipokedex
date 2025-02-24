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
        // Estilo global para simular Windows Vista Aero
        let mut style = (*ctx.style()).clone();
        
        // Colores base de Windows Vista Aero
        let vista_blue = egui::Color32::from_rgb(80, 130, 200);        // Azul base de Vista
        let vista_light_blue = egui::Color32::from_rgb(220, 235, 250); // Azul claro para fondos
        let vista_panel_bg = egui::Color32::from_rgba_premultiplied(240, 248, 255, 220); // Color de panel semi-transparente
        
        // Aplicar estilos de Windows Vista
        style.visuals.window_fill = vista_blue;
        style.visuals.panel_fill = vista_panel_bg;
        style.visuals.widgets.noninteractive.bg_fill = vista_panel_bg;
        style.visuals.widgets.inactive.bg_fill = vista_light_blue;
        style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(230, 240, 255);
        style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(210, 230, 250);
        
        // Sombras y bordes estilo Aero
        style.visuals.window_shadow = egui::epaint::Shadow {
            extrusion: 10.0,
            color: egui::Color32::from_rgba_premultiplied(0, 0, 0, 40),
        };
        
        // Bordes redondeados para todos los widgets
        style.visuals.window_rounding = egui::Rounding::same(8.0);
        style.visuals.menu_rounding = egui::Rounding::same(6.0);
        style.visuals.widgets.noninteractive.rounding = egui::Rounding::same(4.0);
        style.visuals.widgets.inactive.rounding = egui::Rounding::same(4.0);
        style.visuals.widgets.hovered.rounding = egui::Rounding::same(4.0);
        style.visuals.widgets.active.rounding = egui::Rounding::same(4.0);
        
        ctx.set_style(style);

        // Panel central con fondo degradado
        egui::CentralPanel::default()
            .frame(egui::Frame::none()
                .fill(egui::Color32::from_rgb(230, 240, 250))
                .shadow(egui::epaint::Shadow {
                    extrusion: 0.0,
                    color: egui::Color32::TRANSPARENT,
                }))
            .show(ctx, |ui| {
                // Header
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    ui.heading(
                        egui::RichText::new("PokéDex")
                            .size(40.0)
                            .color(egui::Color32::from_rgb(40, 110, 80))
                            .strong()
                    );
                    ui.label(
                        egui::RichText::new("GRAAWW POKEDEX EN RUST")
                            .size(18.0)
                            .color(egui::Color32::from_rgb(60, 130, 100))
                            .strong()
                    );
                    ui.add_space(20.0);
                });

                // Barra de búsqueda estilo Vista
                render_vista_search_bar(ui, &mut self.search_term);
                ui.add_space(20.0);

                // Contenido principal con 2 paneles
                ui.horizontal(|ui| {
                    // Panel izquierdo - Lista de Pokémon
                    render_pokemon_list_panel(ui, &self.database, &self.search_term, &mut self.selected_pokemon);
                    
                    ui.add_space(20.0);
                    
                    // Panel derecho - Detalles del Pokémon
                    render_pokemon_details_panel(ui, &self.database, self.selected_pokemon);
                });
            });
    }
}

fn render_vista_search_bar(ui: &mut egui::Ui, search_term: &mut String) {
    ui.vertical_centered(|ui| {
        let search_bar_frame = egui::Frame::none()
            .fill(egui::Color32::from_rgba_premultiplied(255, 255, 255, 220))
            .rounding(egui::Rounding::same(20.0))
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(180, 200, 220)))
            .shadow(egui::epaint::Shadow {
                extrusion: 4.0,
                color: egui::Color32::from_rgba_premultiplied(0, 0, 0, 30),
            });
            
        search_bar_frame.show(ui, |ui| {
            ui.set_width(ui.available_width() * 0.7);
            ui.horizontal(|ui| {
                ui.label("🔍");
                ui.add_sized(
                    [ui.available_width() - 30.0, 24.0],
                    egui::TextEdit::singleline(search_term)
                        .hint_text("Buscar Pokémon...")
                        .text_color(egui::Color32::from_rgb(40, 70, 110))
                        .frame(false)
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
    let panel_frame = egui::Frame::none()
        .fill(egui::Color32::from_rgba_premultiplied(255, 255, 255, 200))
        .rounding(egui::Rounding::same(8.0))
        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(180, 200, 220)))
        .shadow(egui::epaint::Shadow {
            extrusion: 6.0,
            color: egui::Color32::from_rgba_premultiplied(0, 0, 0, 40),
        });
        
    panel_frame.show(ui, |ui| {
        ui.set_width(300.0);
        ui.set_min_height(500.0);
        ui.vertical(|ui| {
            // Título del panel
            let title_frame = egui::Frame::none()
                .fill(egui::Color32::from_rgb(230, 240, 250))
                .rounding(egui::Rounding::same(6.0))
                .inner_margin(egui::style::Margin::symmetric(8.0, 6.0));
                
            title_frame.show(ui, |ui| {
                ui.heading(
                    egui::RichText::new("Pokémon")
                        .size(24.0)
                        .color(egui::Color32::from_rgb(40, 110, 80))
                        .strong()
                );
            });
            
            ui.add_space(10.0);
            
            // Lista de Pokémon estilo Vista
            egui::ScrollArea::vertical()
                .max_height(450.0)
                .show(ui, |ui| {
                    for pokemon in database.get_all_pokemon() {
                        if pokemon.name.to_lowercase().contains(&search_term.to_lowercase()) {
                            let is_selected = Some(pokemon.id) == *selected_pokemon;
                            
                            // Botón estilo Vista Aero
                            let button_frame = if is_selected {
                                egui::Frame::none()
                                    .fill(egui::Color32::from_rgb(210, 230, 250))
                                    .rounding(egui::Rounding::same(4.0))
                                    .inner_margin(egui::style::Margin::same(6.0))
                                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(120, 170, 220)))
                                    .shadow(egui::epaint::Shadow {
                                        extrusion: 1.0,
                                        color: egui::Color32::from_rgba_premultiplied(0, 0, 0, 20),
                                    })
                            } else {
                                egui::Frame::none()
                                    .fill(egui::Color32::TRANSPARENT)
                                    .rounding(egui::Rounding::same(4.0))
                                    .inner_margin(egui::style::Margin::same(6.0))
                            };
                            
                            button_frame.show(ui, |ui| {
                                let response = ui.add(
                                    egui::Button::new(
                                        egui::RichText::new(format!("#{:03} - {}", pokemon.id, pokemon.name))
                                            .color(if is_selected {
                                                egui::Color32::from_rgb(30, 90, 150)
                                            } else {
                                                egui::Color32::from_rgb(60, 80, 100)
                                            })
                                            .size(16.0)
                                    )
                                    .fill(egui::Color32::TRANSPARENT)
                                    .frame(false)
                                );
                                
                                if response.clicked() {
                                    *selected_pokemon = Some(pokemon.id);
                                }
                            });
                            
                            ui.add_space(2.0);
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
    let panel_frame = egui::Frame::none()
        .fill(egui::Color32::from_rgba_premultiplied(255, 255, 255, 200))
        .rounding(egui::Rounding::same(8.0))
        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(180, 200, 220)))
        .shadow(egui::epaint::Shadow {
            extrusion: 6.0,
            color: egui::Color32::from_rgba_premultiplied(0, 0, 0, 40),
        });
        
    panel_frame.show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.set_min_height(500.0);
        
        if let Some(id) = selected_pokemon {
            if let Some(pokemon) = database.get_pokemon(id) {
                // Nombre y número del Pokémon con estilo Vista
                let title_frame = egui::Frame::none()
                    .fill(egui::Color32::from_rgb(230, 240, 250))
                    .rounding(egui::Rounding::same(6.0))
                    .inner_margin(egui::style::Margin::symmetric(8.0, 6.0));
                    
                title_frame.show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading(
                            egui::RichText::new(&pokemon.name)
                                .size(32.0)
                                .color(egui::Color32::from_rgb(40, 110, 80))
                                .strong()
                        );
                        ui.label(
                            egui::RichText::new(format!("#{:03}", pokemon.id))
                                .size(18.0)
                                .color(egui::Color32::from_rgb(60, 130, 100))
                        );
                    });
                });
                
                ui.add_space(15.0);
                
                // Información básica y estadísticas en un panel con estilo Vista Aero
                let content_frame = egui::Frame::none()
                    .fill(egui::Color32::from_rgba_premultiplied(245, 250, 255, 220))
                    .rounding(egui::Rounding::same(6.0))
                    .inner_margin(egui::style::Margin::same(15.0))
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(200, 220, 240)))
                    .shadow(egui::epaint::Shadow {
                        extrusion: 2.0,
                        color: egui::Color32::from_rgba_premultiplied(0, 0, 0, 20),
                    });
                    
                content_frame.show(ui, |ui| {
                    // Información básica con estilo
                    ui.vertical(|ui| {
                        // Altura y peso con etiquetas en negrita
                        ui.horizontal(|ui| {
                            ui.strong("Altura:");
                            ui.monospace(format!("{:.1} m", pokemon.height as f32 / 10.0));
                            
                            ui.add_space(20.0);
                            
                            ui.strong("Peso:");
                            ui.monospace(format!("{:.1} kg", pokemon.weight as f32 / 10.0));
                        });
                        
                        ui.add_space(10.0);
                        
                        // Tipos con estilo Visual Aero
                        ui.horizontal(|ui| {
                            ui.strong("Tipos:");
                            ui.add_space(10.0);
                            
                            for type_info in &pokemon.types {
                                render_vista_type_badge(ui, type_info);
                                ui.add_space(5.0);
                            }
                        });
                        
                        ui.add_space(20.0);
                        
                        // Separador estilo Vista
                        let separator_color = egui::Color32::from_rgb(180, 200, 220);
                        ui.separator(egui::Separator::horizontal().spacing(10.0).color(separator_color));
                        ui.add_space(20.0);
                        
                        // Sección de estadísticas
                        ui.heading(
                            egui::RichText::new("Estadísticas")
                                .size(20.0)
                                .color(egui::Color32::from_rgb(60, 100, 140))
                                .strong()
                        );
                        ui.add_space(10.0);
                        
                        // Barras de estadísticas estilo Vista Aero
                        render_vista_stat_bar(ui, "HP", pokemon.stats.hp);
                        render_vista_stat_bar(ui, "Ataque", pokemon.stats.attack);
                        render_vista_stat_bar(ui, "Defensa", pokemon.stats.defense);
                        render_vista_stat_bar(ui, "Atq. Esp.", pokemon.stats.special_attack);
                        render_vista_stat_bar(ui, "Def. Esp.", pokemon.stats.special_defense);
                        render_vista_stat_bar(ui, "Velocidad", pokemon.stats.speed);
                    });
                });
            }
        } else {
            // Mensaje cuando no hay selección
            ui.vertical_centered(|ui| {
                ui.add_space(150.0);
                ui.label(
                    egui::RichText::new("Selecciona un Pokémon")
                        .size(24.0)
                        .color(egui::Color32::from_rgb(100, 140, 180))
                );
                
                ui.add_space(20.0);
                ui.label(
                    egui::RichText::new("⚡ 🔍 🌟")
                        .size(28.0)
                        .color(egui::Color32::from_rgb(80, 120, 180))
                );
            });
        }
    });
}

fn render_vista_type_badge(ui: &mut egui::Ui, pokemon_type: &PokemonType) {
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
        _ => egui::Color32::from_rgb(180, 180, 180),
    };

    // Insignia de tipo con estilo Aero Glass
    let badge_frame = egui::Frame::none()
        .fill(type_color)
        .rounding(egui::Rounding::same(12.0))
        .inner_margin(egui::style::Margin::symmetric(8.0, 4.0))
        .shadow(egui::epaint::Shadow {
            extrusion: 2.0,
            color: egui::Color32::from_rgba_premultiplied(0, 0, 0, 60),
        });
        
    badge_frame.show(ui, |ui| {
        ui.label(
            egui::RichText::new(format!("{}", pokemon_type))
                .color(egui::Color32::WHITE)
                .strong()
                .size(14.0)
        );
    });
}

fn render_vista_stat_bar(ui: &mut egui::Ui, name: &str, value: u8) {
    ui.horizontal(|ui| {
        // Nombre de la estadística estilo Vista
        ui.add_sized(
            [80.0, 20.0],
            egui::Label::new(
                egui::RichText::new(name)
                    .color(egui::Color32::from_rgb(60, 100, 140))
                    .strong()
                    .size(15.0)
            )
        );
        
        let progress = value as f32 / 255.0;
        
        // Fondo de la barra estilo Vista
        let bar_bg = egui::Color32::from_rgb(220, 230, 240);
        let bar_frame = egui::Frame::none()
            .fill(bar_bg)
            .rounding(egui::Rounding::same(4.0))
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(180, 200, 220)))
            .inner_margin(egui::style::Margin::same(2.0));
            
        bar_frame.show(ui, |ui| {
            ui.allocate_space(egui::Vec2::new(ui.available_width() - 50.0, 20.0));
            
            // Calcular el color de la barra basado en el valor
            let bar_color = egui::Color32::from_rgb(
                (200.0 - progress * 100.0) as u8,
                (180.0 + progress * 75.0) as u8,
                (140.0 + progress * 40.0) as u8,
            );
            
            // Dibujar la barra de progreso
            let bar_width = (ui.available_width() - 4.0) * progress;
            let bar_height = 16.0;
            let bar_pos = ui.min_rect().min + egui::Vec2::new(2.0, 2.0);
            
            if bar_width > 0.0 {
                let bar_rect = egui::Rect::from_min_size(
                    bar_pos, 
                    egui::Vec2::new(bar_width, bar_height)
                );
                
                // Degradado de la barra estilo Vista
                ui.painter().rect_filled(
                    bar_rect,
                    egui::Rounding::same(3.0),
                    bar_color
                );
                
                // Efecto de brillo en la parte superior (característico de Vista)
                ui.painter().line_segment(
                    [
                        bar_rect.left_top() + egui::Vec2::new(1.0, 1.0),
                        bar_rect.right_top() + egui::Vec2::new(-1.0, 1.0)
                    ],
                    egui::Stroke::new(1.0, egui::Color32::from_rgba_premultiplied(255, 255, 255, 180))
                );
            }
        });
        
        // Valor numérico estilo Vista
        ui.add_sized(
            [30.0, 20.0],
            egui::Label::new(
                egui::RichText::new(value.to_string())
                    .color(egui::Color32::from_rgb(60, 100, 140))
                    .monospace()
                    .size(15.0)
            )
        );
    });
    
    ui.add_space(5.0);
}