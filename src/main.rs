#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

use eframe::egui::{CollapsingHeader, Event, Id, PointerButton, Ui, Window};

use component::TableNode;
use data_source::DataSource;

use crate::component::NodeContextMenu;

mod component;
mod data_source;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    eframe::run_native(
        "Mysql Graph Editor",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )?;
    Ok(())
}

pub struct MyApp {
    current_url_input: String,
    data_sources: Arc<Mutex<Vec<DataSource>>>,
    nodes: Vec<TableNode>,
    scale: f32,
    node_context_menu: NodeContextMenu,
    show_node_context_menu: bool,
}

impl MyApp {
    fn new(_: &eframe::CreationContext<'_>) -> Self {
        Self {
            current_url_input: String::new(),
            data_sources: Arc::new(Mutex::new(vec![])),
            nodes: vec![],
            scale: 0.8,
            node_context_menu: NodeContextMenu::default(),
            show_node_context_menu: false,
        }
    }
    fn add_data_source(&mut self, url: &str) {
        let data_sources = self.data_sources.clone();
        let url = url.to_string();
        tokio::spawn(async move {
            // ⚠️ 注意先执行完async的任务，然后再lock变量进行修改
            let mut data_source = DataSource::new(&url).await.unwrap();
            data_source.flash_schemas();
            let mut lock = data_sources.lock().unwrap();
            lock.push(data_source);
        });
    }
    fn update_scale(&mut self, value: f32) {
        self.scale += value;
        if self.scale < 0.2 {
            self.scale = 0.2;
        } else if self.scale > 1.5 {
            self.scale = 1.5
        }
    }
    fn data_sources_ui(&mut self, ui: &mut Ui) {
        for data_source in self.data_sources.lock().unwrap().iter_mut() {
            CollapsingHeader::new(&data_source.url)
                .default_open(false)
                .show(ui, |ui| {
                    if ui.button("flash schemas").clicked() {
                        data_source.flash_schemas();
                    }
                    for (schema_name, tables) in data_source.get_schemas().iter() {
                        CollapsingHeader::new(schema_name)
                            .default_open(false)
                            .show(ui, |ui| {
                                if ui.button(format!("flash {schema_name} tables")).clicked() {
                                    data_source.flash_tables(schema_name);
                                }
                                for (table_name, columns) in tables.iter() {
                                    CollapsingHeader::new(table_name).default_open(false).show(
                                        ui,
                                        |ui| {
                                            if ui
                                                .button(format!("flash {table_name} columns"))
                                                .clicked()
                                            {
                                                data_source
                                                    .flash_table_columns(schema_name, table_name);
                                            }
                                            for column in columns {
                                                ui.label(&column.field);
                                            }
                                        },
                                    );
                                }
                            });
                    }
                });
        }
        ui.text_edit_singleline(&mut self.current_url_input);
        if ui.button("add data source").clicked() {
            let current_url_input = self.current_url_input.clone();
            self.current_url_input.clear();
            self.add_data_source(current_url_input.as_str());
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
        ctx.input(|input_state| {
            if input_state.zoom_delta() > 1.0 {
                self.update_scale(0.02);
            } else if input_state.zoom_delta() < 1.0 {
                self.update_scale(-0.02);
            }
            for x in input_state.events.iter() {
                match x {
                    Event::PointerButton {
                        pos,
                        button,
                        pressed,
                        ..
                    } => match button {
                        PointerButton::Primary => {
                            if *pressed {
                                if !self.node_context_menu.contains(*pos) {
                                    self.show_node_context_menu = false;
                                }
                            }
                        }
                        PointerButton::Secondary => {
                            if *pressed {
                                self.node_context_menu.update_pos(*pos);
                                self.show_node_context_menu = true;
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        });
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            let mut window = Window::new("Schemas")
                .id(Id::new("Schemas"))
                .resizable(true)
                .collapsible(true)
                .vscroll(true)
                .title_bar(true);
            let mut open = true;
            window = window.open(&mut open);
            window.show(ctx, |ui| self.data_sources_ui(ui));
            if self.show_node_context_menu {
                ui.add(&mut self.node_context_menu);
                if let Some(_) = self.node_context_menu.if_clicked(ui) {
                    // if let Some(_) = &self.data_source {
                    //     // for (table_name, columns) in data_source.get_schemas().iter() {
                    //     //     // self.nodes.push(TableNode::new(table_name, columns.clone()));
                    //     //     break;
                    //     // }
                    // }
                    self.show_node_context_menu = false;
                }
            }
            for (index, x) in self.nodes.iter_mut().enumerate() {
                x.scale = self.scale;
                ui.push_id(index, |ui| {
                    ui.add(x);
                });
            }
        });
    }
}
