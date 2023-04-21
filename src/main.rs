#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use egui_extras::{Column, TableBuilder};
use mysql_async::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod cache;
use cache::{Cache, TableColumn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    eframe::run_native(
        "Mysql Graph Editor",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )?;
    Ok(())
}

struct MyApp {
    mysql_url: String,
    conn: Arc<Mutex<Option<mysql_async::Conn>>>,
    cache: Arc<Mutex<Cache>>,
}

impl MyApp {
    fn new(_: &eframe::CreationContext<'_>) -> Self {
        Self {
            mysql_url: String::new(),
            conn: Arc::new(Mutex::new(None)),
            cache: Arc::new(Mutex::new(Cache::default())),
        }
    }
    fn try_con(&mut self) {
        let conn_arc = self.conn.clone();
        let cache_arc = self.cache.clone();
        let mysql_url = self.mysql_url.clone();
        tokio::spawn(async move {
            if let Ok(mut conn) = mysql_async::Pool::new(mysql_url.as_str()).get_conn().await {
                let mut tables: HashMap<String, Vec<TableColumn>> = HashMap::new();
                if let Ok(table_names) = "show tables"
                    .with(())
                    .map(&mut conn, |table: String| table)
                    .await
                {
                    for table_name in table_names {
                        if let Ok(columns) = format!("show columns from {table_name}")
                            .as_str()
                            .with(())
                            .map(
                                &mut conn,
                                |(field, type_, null, key, default, extra): (
                                    String,
                                    String,
                                    String,
                                    String,
                                    Option<String>,
                                    String,
                                )| {
                                    TableColumn {
                                        field,
                                        type_,
                                        null,
                                        key,
                                        default: default.unwrap_or(String::new()),
                                        extra,
                                    }
                                },
                            )
                            .await
                        {
                            tables.insert(table_name, columns);
                        }
                    }
                }
                if let Ok(mut cache_mutex) = cache_arc.lock() {
                    cache_mutex.tables = tables;
                }
                if let Ok(mut mutex_lock) = conn_arc.lock() {
                    *mutex_lock = Some(conn);
                }
            }
        });
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Data Source");
            ui.horizontal(|ui| {
                ui.label("URL: ");
                ui.text_edit_singleline(&mut self.mysql_url);
                if ui.button("Connect").clicked() {
                    self.try_con();
                }
            });
            if let Ok(cache) = self.cache.lock() {
                let tables = &cache.tables;
                for (index, (table_name, columns)) in tables.iter().enumerate() {
                    ui.separator();
                    ui.heading(table_name);
                    ui.push_id(index, |ui| {
                        let table = TableBuilder::new(ui)
                            .striped(true)
                            .column(Column::initial(100.0))
                            .column(Column::initial(100.0));
                        table
                            .header(20.0, |mut header| {
                                header.col(|ui| {
                                    ui.strong("Field");
                                });
                                header.col(|ui| {
                                    ui.strong("Type");
                                });
                            })
                            .body(|mut body| {
                                for column in columns {
                                    body.row(20.0, |mut row| {
                                        row.col(|ui| {
                                            ui.label(&column.field);
                                        });
                                        row.col(|ui| {
                                            ui.label(&column.type_);
                                        });
                                    });
                                }
                            });
                    });
                }
            }
        });
    }
}
