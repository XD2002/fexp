use std::fs;
use std::path::PathBuf;
use eframe::egui::{self, CentralPanel};
use eframe::{self, App};
use egui::{ComboBox, Id, ScrollArea, SidePanel, TopBottomPanel};
use crate::conversion::format_bytes;
use crate::icon::get_icon;
use crate::navigation::Navigator;
use crate::file_ops::{delete_file, get_file_type, list_directory_contents, open_file, open_file_with};
use crate::search::search_file_in_dir;
use crate::settings::Settings;
use crate::sort::{Alphabetical, AlphabeticalDirectoriesFirst};

pub struct FExpApp {
    navigator: Navigator,
    focussed_file: PathBuf,
    settings: Settings,
    search_query: String,
    search_res: Vec<PathBuf>,
    search_handle: Option<std::thread::JoinHandle<Vec<PathBuf>>>,
    root_menu_open: bool,
    submenu_open: bool,
    submenu_anchor: Option<egui::Rect>,
}

impl Default for FExpApp {
    fn default() -> Self {
        Self {
            navigator: Navigator::new(),
            focussed_file: PathBuf::default(),
            settings: Settings::load(),
            search_query: String::new(),
            search_res: Vec::new(),
            search_handle: None,
            root_menu_open: false,
            submenu_open: false,
            submenu_anchor: None,
        }
    }
}

impl eframe::App for FExpApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        if let Some(handle) = &self.search_handle {
            if handle.is_finished() {
                if let Ok(results) = self.search_handle.take().unwrap().join() {
                    self.search_res = results;
                }
            }
        }

        self.root_menu_open = false;

        egui_extras::install_image_loaders(ctx);

        let mut files = list_directory_contents(&self.navigator.current_path());
        files = (*self.settings.sorting_strategy).sort(files);

        TopBottomPanel::top("topbar").exact_height(64.0).show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("back").clicked() {
                    self.navigator.go_back_one();
                }
                if ui.button("forward").clicked() {
                    self.navigator.go_forward_one();
                }
                ComboBox::from_id_salt(0)
                    .selected_text("sort")
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.settings.sorting_strategy, Box::new(Alphabetical), "alphabetical");
                        ui.selectable_value(&mut self.settings.sorting_strategy, Box::new(AlphabeticalDirectoriesFirst), "directories first");
                    }
                );
                ui.checkbox(&mut self.settings.hide_hidden_files, "hide hidden files");
                let res = ui.text_edit_singleline(&mut self.search_query);
                if res.changed() {
                    self.search_res = Vec::new();
                    self.search_handle = Some(search_file_in_dir(&self.search_query, &self.navigator.current_path().to_string_lossy().to_string()));
                }
            })
        });

        SidePanel::right("search_pane").exact_width(if !self.search_query.is_empty() {256.0} else {0.0}).show(ctx, |ui| {
            ui.heading("Search");

            ScrollArea::vertical().show(ui, |ui| {
                for (index, full_path) in self.search_res.iter().enumerate() {
                    let file = full_path.file_name().unwrap().to_string_lossy();
                    if !self.settings.hide_hidden_files || !file.starts_with('.') {
                        ui.push_id(index, |ui| {
                            let file_type = get_file_type(&full_path);
                            let icon = get_icon(file_type);

                            let response = ui.horizontal(|ui| {
                                ui.add(
                                    egui::Image::new(icon)
                                        .max_width(16.0)
                                        .corner_radius(1.0),
                                );

                                ui.label(file.clone());
                            }).response.interact(egui::Sense::click());

                            if response.clicked() {
                                self.focussed_file = full_path.to_path_buf();
                            }

                            if response.double_clicked() {
                                if let Ok(metadata) = fs::metadata(full_path.clone()) {
                                    if metadata.is_file() {
                                        open_file(&full_path);
                                    } else if metadata.is_dir() {
                                        self.navigator.navigate_to(&full_path);
                                    }
                                }
                            }
                        });
                    }
                }
            });
        });

        SidePanel::right("focussed_file_panel").exact_width(256.0).show(ctx, |ui| {
            ui.heading("Details");
            if let Some(filename) = self.focussed_file.file_name() {
                ui.label(format!("name: {}", filename.to_string_lossy()));
            }
            if let Ok(metadata) = fs::metadata(self.focussed_file.clone()) {
                ui.label(format!("size: {}", format_bytes(metadata.len())));
            }
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.heading(self.navigator.current_path().to_string_lossy());

            ScrollArea::vertical().show(ui, |ui| {
                for (index, full_path) in files.iter().enumerate() {
                    let file = full_path.file_name().unwrap().to_string_lossy();
                    if !self.settings.hide_hidden_files || !file.starts_with('.') {
                        ui.push_id(index, |ui| {
                            let file_type = get_file_type(&full_path);
                            let icon = get_icon(file_type);

                            let response = ui.horizontal(|ui| {
                                ui.add(
                                    egui::Image::new(icon)
                                        .max_width(16.0)
                                        .corner_radius(1.0),
                                );

                                ui.label(file.clone());
                            }).response.interact(egui::Sense::click());

                            if response.clicked() {
                                self.focussed_file = full_path.to_path_buf();
                            }

                            if response.double_clicked() {
                                if let Ok(metadata) = fs::metadata(full_path.clone()) {
                                    if metadata.is_file() {
                                        open_file(&full_path);
                                    } else if metadata.is_dir() {
                                        self.navigator.navigate_to(&full_path);
                                    }
                                }
                            }

                            response.context_menu(|ui| {
                                self.root_menu_open = true;

                                if ui.button("Delete").clicked() {
                                    delete_file(full_path);
                                    ui.close();
                                    self.submenu_open = false;
                                    self.submenu_anchor = None;
                                }

                                let open_but = ui.button("Open with...");

                                self.submenu_anchor = Some(open_but.rect);

                                if open_but.hovered() {
                                    self.submenu_open = true;
                                }

                                if open_but.clicked() {
                                    self.submenu_open = !self.submenu_open;
                                }
                            });

                            if !self.root_menu_open {
                                self.submenu_open = false;
                                self.submenu_anchor = None;
                            }

                            if self.submenu_open {
                                if let Some(anchor) = self.submenu_anchor {
                                    let submenu_pos = anchor.right_top() + egui::vec2(6.0, 0.0);

                                    let area_response = egui::Area::new(Id::new("open_with_menu"))
                                        .fixed_pos(submenu_pos)
                                        .order(egui::Order::Foreground)
                                        .show(ctx, |ui| {
                                            egui::Frame::popup(ui.style()).show(ui, |ui| {
                                                if ui.button("Shotwell").clicked() {
                                                    open_file_with(&full_path, &"shotwell".to_string());
                                                    self.submenu_open = false
                                                }
                                            });
                                        }).response;

                                    let pointer_pos = ctx.input(|i| i.pointer.hover_pos());

                                    let mut still_hovering = area_response.hovered();

                                    if let Some(ptr) = pointer_pos {
                                        let inflation = 4.0;
                                        let anchor_inflated = egui::Rect::from_min_max(
                                            anchor.min - egui::vec2(inflation, inflation),
                                            anchor.max + egui::vec2(inflation, inflation),
                                        );
                                        let area_rect_inflated = egui::Rect::from_min_max(
                                            area_response.rect.min - egui::vec2(inflation, inflation),
                                            area_response.rect.max + egui::vec2(inflation, inflation),
                                        );

                                        if anchor_inflated.contains(ptr) || area_rect_inflated.contains(ptr) {
                                            still_hovering = true;
                                        } else {
                                            let left = anchor.right().min(area_response.rect.left());
                                            let right = anchor.right().max(area_response.rect.left());
                                            let top = anchor.top().min(area_response.rect.top());
                                            let bottom = anchor.bottom().max(area_response.rect.bottom());
                                            let bridge = egui::Rect::from_min_max(
                                                egui::pos2(left - 8.0, top - 2.0),
                                                egui::pos2(right + 8.0, bottom + 2.0),
                                            );
                                            if bridge.contains(ptr) {
                                                still_hovering = true;
                                            }
                                        }
                                    }

                                    if !still_hovering {
                                        self.submenu_open = false;
                                        self.submenu_anchor = None;
                                    }
                                } else {
                                    self.root_menu_open = false;
                                }
                            }
                        });
                    }
                }
            });
        });
    }
}
