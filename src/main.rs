use eframe::egui::{
        self, style::Selection, Color32, FontFamily, FontId, Key, RichText, Stroke, TextEdit,
};

use std::sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
};
use std::{thread, time::Duration};

pub mod config;
pub mod constants;
pub mod launch_app;
pub mod loadfont;
pub mod structs;
use crate::constants::*;
use crate::structs::*;

fn main() {
        // ==== import config ====
        let conf: Config;
        let conf_created: bool;
        (conf, conf_created) = config::import();

        // ==== program ====
        let vp_width = conf.window.width;
        let vp_height = conf.window.height;
        let viewport = egui::ViewportBuilder::default()
                .with_decorations(false)
                .with_app_id(APP_NAME)
                .with_title(APP_NAME.to_owned())
                .with_movable_by_background(true)
                .with_always_on_top()
                .with_inner_size(egui::vec2(vp_width as f32, vp_height as f32));

        let native_options = eframe::NativeOptions {
                viewport,
                centered: true,
                ..Default::default()
        };

        let search_focus = true;
        let sorted_apps: Vec<App> = Vec::new();
        let _ = eframe::run_native(
                APP_NAME,
                native_options,
                Box::new(|cc| {
                        Ok(Box::new(KastLauncherApp::new(
                                conf,
                                conf_created,
                                search_focus,
                                sorted_apps,
                                cc,
                        )))
                }),
        );
}

struct KastLauncherApp {
        config: Config,
        search: String,
        conf_created: bool,
        search_focus: bool,
        sorted_apps: Vec<App>,
        selected_index: usize,
        is_quitting: Arc<AtomicBool>,
        threads: Vec<std::thread::JoinHandle<()>>,
}

impl KastLauncherApp {
        fn new(
                mut config: Config,
                conf_created: bool,
                search_focus: bool,
                sorted_apps: Vec<App>,
                cc: &eframe::CreationContext<'_>,
        ) -> Self {
                setup_custom_style(&cc.egui_ctx, &mut config);
                loadfont::replace_fonts(&cc.egui_ctx, config.font.path.clone());
                loadfont::add_font(&cc.egui_ctx, config.font.path.clone());

                Self {
                        config,
                        search: String::new(),
                        conf_created,
                        search_focus,
                        sorted_apps,
                        selected_index: 0,
                        is_quitting: Arc::new(AtomicBool::new(false)),
                        threads: vec![],
                }
        }
}

impl KastLauncherApp {
        fn search_loop(&mut self) {
                self.sorted_apps.clear();
                let search_l = self.search.to_lowercase();
                self.sorted_apps.extend(
                        self.config
                                .apps
                                .iter()
                                .filter(|app| app.name.to_lowercase().contains(&search_l))
                                .cloned(),
                );
        }
        fn quit(&mut self) {
                for handle in self.threads.drain(..) {
                        let _ = handle.join();
                }
                std::process::exit(0);
        }

        fn input(&mut self, ctx: &egui::Context) {
                // escape: exit program
                if ctx.input(|i| i.key_pressed(Key::Escape)) {
                        self.is_quitting.store(true, Ordering::SeqCst);
                }
                // enter: toggle searchbar focus
                if ctx.input(|i| i.key_pressed(Key::Enter)) {
                        self.search_focus = false
                }
                // x: close popup
                if ctx.input(|i| i.key_pressed(Key::X)) {
                        self.conf_created = false;
                }
        }
        fn timeout_logic(&mut self) {
                let timeout_q = Arc::clone(&self.is_quitting);
                let timeo = self.config.misc.timeout;
                let _ = thread::spawn(move || {
                        thread::sleep(Duration::from_secs(timeo));
                        timeout_q.store(true, Ordering::SeqCst);
                });
        }
}

impl eframe::App for KastLauncherApp {
        fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
                self.timeout_logic();
                self.search_loop();
                self.input(ctx);

                // ==== ui =====
                egui::CentralPanel::default().show(ctx, |ui| {
                        let corner_rad = egui::CornerRadius::same(self.config.window.elem_cnr_rad);
                        let available_width = ui.available_width();
                        let search_bar = egui::Frame::new().corner_radius(corner_rad);
                        let app_list = egui::Frame::new().corner_radius(corner_rad);

                        search_bar.show(ui, |ui| {
                                ui.horizontal(|ui| {
                                        let _ = ui.allocate_ui_with_layout(
                                                egui::vec2(ui.available_width(), self.config.window.row_height),
                                                egui::Layout::left_to_right(egui::Align::Center),
                                                |ui| {
                                                        ui.painter().rect_filled(
                                                                ui.max_rect(),
                                                                self.config.window.elem_cnr_rad,
                                                                ui.visuals().selection.bg_fill,
                                                        );
                                                        ui.add_space(12.0); // padding for icon
                                                        if self.config.icons.entry_icon {
                                                                ui.label(
                                                                        RichText::new(format!("{}", self.config.icons.entry)).font(
                                                                                FontId::new(
                                                                                        self.config.font.size + 7.0,
                                                                                        FontFamily::Monospace,
                                                                                ),
                                                                        ),
                                                                );
                                                        }

                                                        let field = ui.add(
                                                                TextEdit::singleline(&mut self.search)
                                                                        .font(FontId::new(self.config.font.size, FontFamily::default()))
                                                                        .frame(false)
                                                                        .background_color(hex_to_color32(&self.config.colors.accent))
                                                                        .hint_text(
                                                                                RichText::new(&self.config.misc.search_hint)
                                                                                        .color(hex_to_color32(&self.config.colors.text_aux)),
                                                                        )
                                                                        .desired_width(available_width),
                                                        );
                                                        if self.search_focus == true {
                                                                if self.conf_created == false {
                                                                        field.request_focus();
                                                                }
                                                        }
                                                },
                                        );
                                });
                        });

                        app_list.show(ui, |ui| {
                                // ==== list navigation ====
                                let max = self.sorted_apps.len().saturating_sub(1);
                                ui.input(|i| {
                                        if i.key_pressed(egui::Key::ArrowDown) {
                                                self.selected_index = (self.selected_index + 1).min(max);
                                        }
                                        if i.key_pressed(egui::Key::ArrowUp) {
                                                self.selected_index = self.selected_index.saturating_sub(1);
                                        }

                                        if i.key_pressed(egui::Key::Enter) || i.key_pressed(egui::Key::Space) {
                                                let app = self.sorted_apps[self.selected_index].clone();
                                                let app_q = Arc::clone(&self.is_quitting);
                                                let t_launch_app = thread::spawn(move || {
                                                        launch_app::run(app);
                                                        app_q.store(true, Ordering::SeqCst);
                                                });
                                                self.threads.push(t_launch_app);
                                        }
                                });

                                ui.add_space(4.0); // padding between search and list
                                egui::ScrollArea::vertical()
                                        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                                        .show(ui, |ui| {
                                                for (idx, app) in self.sorted_apps.iter().enumerate() {
                                                        let selected = idx == self.selected_index;

                                                        let bg_color = if selected {
                                                                ui.visuals().selection.bg_fill
                                                        } else {
                                                                egui::Color32::TRANSPARENT
                                                        };

                                                        let _ = ui.allocate_ui_with_layout(
                                                                egui::vec2(ui.available_width(), self.config.window.row_height),
                                                                egui::Layout::left_to_right(egui::Align::Center),
                                                                |ui| {
                                                                        ui.painter().rect_filled(
                                                                                ui.max_rect(),
                                                                                self.config.window.elem_cnr_rad,
                                                                                bg_color,
                                                                        );

                                                                        ui.label(RichText::new(format!(" {}", app.icon)).font(
                                                                                FontId::new(
                                                                                        self.config.font.size + 7.0,
                                                                                        FontFamily::Monospace,
                                                                                ),
                                                                        ));

                                                                        ui.label(RichText::new(&app.name).font(FontId::new(
                                                                                self.config.font.size,
                                                                                FontFamily::default(),
                                                                        )));
                                                                },
                                                        );
                                                }
                                        });
                        });

                        // popup: no config was found and a new one has been created

                        if self.conf_created {
                                egui::Window::new("Info")
                                        .resizable(false)
                                        .title_bar(false)
                                        .collapsible(false)
                                        .show(ctx, |ui| {
                                                ui.vertical_centered(|ui| {
                                                        ui.label(RichText::new(CONF_NOT_FOUND_ICON).font(FontId::new(
                                                                self.config.font.size + 15.0,
                                                                FontFamily::Monospace,
                                                        )));
                                                        ui.label(RichText::new(CONF_NOT_FOUND_TEXT).font(FontId::new(
                                                                self.config.font.size - 2.0,
                                                                FontFamily::default(),
                                                        )))
                                                });
                                        });
                        };

                        if self.is_quitting.load(Ordering::SeqCst) {
                                self.quit()
                        }
                });
        }
}

// ==== style ====

fn hex_to_color32(hex: &str) -> Color32 {
        let hex = hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255);
        Color32::from_rgb(r, g, b)
}

fn setup_custom_style(ctx: &egui::Context, config: &mut Config) {
        ctx.style_mut(|style| {
                style.visuals.hyperlink_color = hex_to_color32(&config.colors.text);
                style.visuals.text_cursor.stroke.color = hex_to_color32(&config.colors.text);
                style.visuals.window_fill = hex_to_color32(&config.colors.background);
                style.visuals.selection = Selection {
                        bg_fill: hex_to_color32(&config.colors.accent),
                        stroke: Stroke::new(1.0, hex_to_color32(&config.colors.accent)),
                };

                style.visuals.override_text_color = Some(hex_to_color32(&config.colors.text));
                style.visuals.panel_fill = hex_to_color32(&config.colors.background);
        });
}
