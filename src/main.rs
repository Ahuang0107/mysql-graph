#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mysql_graph_gui::MysqlGraphApp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    eframe::run_native(
        "Mysql Graph Editor",
        eframe::NativeOptions::default(),
        Box::new(|_| Box::new(MysqlGraphApp::default())),
    )?;
    Ok(())
}
