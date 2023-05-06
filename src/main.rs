#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;

use eframe::egui::{Event, PointerButton};
use mysql_async::prelude::Query;
use mysql_async::{Conn, Pool};

use component::NodeContextMenu;
use component::TableNode;
use model::{ColumnStatement, ColumnsStatement, DbColumnStatement};

mod component;
mod model;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(rename = "mysql-url")]
    mysql_url: String,
}

impl Config {
    pub fn from_yaml(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config = serde_yaml::from_str::<Self>(&content)?;
        Ok(config)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tables = {
        let config = Config::from_yaml("config/config.private.yaml")?;
        let mut tables = HashMap::new();
        let mut conn = Pool::from_url(config.mysql_url)?.get_conn().await?;
        for table in "show tables"
            .fetch::<String, &mut Conn>(&mut conn)
            .await?
            .iter()
        {
            let columns = format!("show columns from `{table}`")
                .fetch::<DbColumnStatement, &mut Conn>(&mut conn)
                .await?
                .into_iter()
                .map(|value| ColumnStatement::from(value))
                .collect::<ColumnsStatement>();
            tables.insert(table.clone(), columns);
        }
        tables
    };
    eframe::run_native(
        "Mysql Graph Editor",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(MyApp::new(cc, tables))),
    )?;
    Ok(())
}

pub struct MyApp {
    tables: HashMap<String, ColumnsStatement>,
    nodes: Vec<TableNode>,
    scale: f32,
    node_context_menu: NodeContextMenu,
    show_node_context_menu: bool,
}

impl MyApp {
    fn new(_: &eframe::CreationContext<'_>, tables: HashMap<String, ColumnsStatement>) -> Self {
        let items = tables.keys().collect::<Vec<&String>>();
        let mut node_context_menu = NodeContextMenu::default();
        node_context_menu.update_items(items);
        Self {
            tables,
            nodes: vec![],
            scale: 0.8,
            node_context_menu,
            show_node_context_menu: false,
        }
    }
    fn update_scale(&mut self, value: f32) {
        self.scale += value;
        if self.scale < 0.2 {
            self.scale = 0.2;
        } else if self.scale > 1.5 {
            self.scale = 1.5
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
            if self.show_node_context_menu {
                ui.add(&mut self.node_context_menu);
                if let Some(table_name) = self.node_context_menu.if_clicked(ui) {
                    self.nodes.push(TableNode::new(
                        table_name.as_str(),
                        self.tables.get(table_name.as_str()).unwrap().clone(),
                    ));
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
