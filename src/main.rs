#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::Vec2;
use mysql_graph_gui::MysqlGraphApp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    eframe::run_native(
        "Mysql Graph Editor",
        eframe::NativeOptions {
            initial_window_size: Some(Vec2::new(1440.0, 720.0)),
            ..Default::default()
        },
        Box::new(|_| Box::new(MysqlGraphApp::default())),
    )?;
    Ok(())
}
