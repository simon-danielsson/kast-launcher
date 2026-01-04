use eframe::egui::{
        self, Color32,
        FontFamily::{self, Monospace},
        FontId, Key, Response, RichText, Stroke, TextEdit,
        style::Selection,
};
use egui::Ui;
use std::sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
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

        // ==== apply colors ====
        let colors: Colors = Colors::new(&conf.colors);
        // ==== program ====
        let vp_width = conf.window.width;
        let vp_height = conf.window.height;
        let viewport = egui::ViewportBuilder::default()
                .with_decorations(false)
                .with_taskbar(false)
                .with_resizable(false)
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
                                colors,
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
        colors: Colors,
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
                config: Config,
                colors: Colors,
                conf_created: bool,
                search_focus: bool,
                sorted_apps: Vec<App>,
                cc: &eframe::CreationContext<'_>,
        ) -> Self {
                loadfont::replace_fonts(&cc.egui_ctx, config.font.path.clone());
                loadfont::add_font(&cc.egui_ctx, config.font.path.clone());
                let is_quitting = Arc::new(AtomicBool::new(false));
                let timeout_q = Arc::clone(&is_quitting);
                let timeo = config.misc.timeout;
                thread::spawn(move || {
                        thread::sleep(Duration::from_secs(timeo));
                        timeout_q.store(true, Ordering::SeqCst);
                });

                Self {
                        config,
                        colors: colors,
                        search: String::new(),
                        conf_created,
                        search_focus,
                        sorted_apps,
                        selected_index: 0,
                        is_quitting: is_quitting,
                        threads: vec![],
                }
        }

        // ==== program logic ====

        fn search_loop(&mut self) {
                self.sorted_apps.clear();
                let search_l = self.search.to_lowercase();
                self.sorted_apps.extend(
                        self.config
                                .apps
                                .iter()
                                .filter(|app| {
                                        app.name.to_lowercase().contains(&search_l)
                                        || app.group.to_lowercase().contains(&search_l)
                                })
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

        // ==== ui ====

        fn add_icon(&mut self, ui: &mut Ui, icon: char, size_offset: Option<f32>) -> Response {
                match size_offset {
                        Some(f) => f,
                        None => ICON_FONTSIZE_OFFSET,
                };
                let rich_text = RichText::new(format!("{}", icon)).font(FontId::new(
                        self.config.font.size + ICON_FONTSIZE_OFFSET,
                        Monospace,
                ));
                ui.label(rich_text)
        }
        fn add_gen_label(&mut self, ui: &mut Ui, text: &String, color: Color32) -> Response {
                let rich_text = RichText::new(text)
                        .color(color)
                        .font(FontId::new(self.config.font.size, FontFamily::default()));
                ui.label(rich_text)
        }
}

impl eframe::App for KastLauncherApp {
        fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
                self.input(ctx);
                setup_custom_style(ctx, &self.colors);

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

                                                        ui.add_space(12.0);
                                                        if self.config.icons.entry_icon {
                                                                self.add_icon(ui, self.config.icons.entry, None);
                                                        }

                                                        let field = ui.add(
                                                                TextEdit::singleline(&mut self.search)
                                                                        .font(FontId::new(self.config.font.size, FontFamily::default()))
                                                                        .frame(false)
                                                                        .background_color(self.colors.accent)
                                                                        .hint_text(
                                                                                RichText::new(&self.config.misc.search_hint)
                                                                                        .color(self.colors.text_aux),
                                                                        )
                                                                        .desired_width(available_width),
                                                        );
                                                        if field.changed() {
                                                                self.search_loop();
                                                        }

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
                                                if let Some(app) = self.sorted_apps.get(self.selected_index).cloned() {
                                                        let app_q = Arc::clone(&self.is_quitting);
                                                        let t_launch_app = thread::spawn(move || {
                                                                launch_app::run(app);
                                                                app_q.store(true, Ordering::SeqCst);
                                                        });
                                                        self.threads.push(t_launch_app);
                                                }
                                        }
                                });

                                ui.add_space(4.0); // padding between search and list

                                let new_app_l_to_please_the_borrow_checker: Vec<_> =
                                self.sorted_apps.iter().cloned().collect();
                                // Scroll area for apps
                                egui::ScrollArea::vertical()
                                        .id_salt("apps_scroll_area")
                                        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                                        .show(ui, |ui| {
                                                for (idx, app) in new_app_l_to_please_the_borrow_checker.iter().enumerate()
                                                {
                                                        let selected = idx == self.selected_index;

                                                        let bg_color = if selected {
                                                                ui.visuals().selection.bg_fill
                                                        } else {
                                                                egui::Color32::TRANSPARENT
                                                        };

                                                        // each row
                                                        let row_response = ui
                                                                .allocate_ui_with_layout(
                                                                        egui::vec2(ui.available_width(), self.config.window.row_height),
                                                                        egui::Layout::left_to_right(egui::Align::Center),
                                                                        |ui| {
                                                                                ui.painter().rect_filled(
                                                                                        ui.max_rect(),
                                                                                        self.config.window.elem_cnr_rad,
                                                                                        bg_color,
                                                                                );
                                                                                ui.add_space(12.0);

                                                                                self.add_icon(ui, app.icon, None);
                                                                                self.add_gen_label(ui, &app.name, self.colors.text);
                                                                                self.add_gen_label(ui, &app.group, self.colors.text_aux)
                                                                        },
                                                                )
                                                        .response;

                                                        // scroll to selected row
                                                        if selected {
                                                                ui.scroll_to_rect(row_response.rect, Some(egui::Align::Center));
                                                        }
                                                }
                                        });
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
                                                self.add_icon(ui, CONF_NOT_FOUND_ICON, Some(15.0));
                                                self.add_gen_label(ui, &CONF_NOT_FOUND_TEXT.to_string(), self.colors.text);
                                        });
                                })
                } else {
                        None
                };

                if self.is_quitting.load(Ordering::SeqCst) {
                        self.quit()
                }
        }
}

// ==== style ====

struct Colors {
        text: Color32,
        text_aux: Color32,
        background: Color32,
        accent: Color32,
}

impl Colors {
        fn new(config_colors: &structs::Colors) -> Self {
                Self {
                        text: hex_to_color32(&config_colors.text),
                        text_aux: hex_to_color32(&config_colors.text_aux),
                        background: hex_to_color32(&config_colors.background),
                        accent: hex_to_color32(&config_colors.accent),
                }
        }
}

fn hex_to_color32(hex: &str) -> Color32 {
        let hex = hex.trim_start_matches('#');
        let mut bytes = [255u8; 3];

        for i in 0..3 {
                if let Ok(val) = u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16) {
                        bytes[i] = val;
                }
        }
        Color32::from_rgb(bytes[0], bytes[1], bytes[2])
}

fn setup_custom_style(ctx: &egui::Context, colors: &Colors) {
        ctx.style_mut(|style| {
                style.visuals.hyperlink_color = colors.text;
                style.visuals.text_cursor.stroke.color = colors.text;
                style.visuals.window_fill = colors.background;
                style.visuals.selection = Selection {
                        bg_fill: colors.accent,
                        stroke: Stroke::new(1.0, colors.accent),
                };

                style.visuals.override_text_color = Some(colors.text);
                style.visuals.panel_fill = colors.background
        });
}
