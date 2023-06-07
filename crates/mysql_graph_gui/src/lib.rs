use eframe::egui::{Color32, Pos2, Stroke, Widget};
use eframe::epaint::CubicBezierShape;

use crate::component::TableNode;

mod component;
mod ui_state;

#[derive(Default)]
pub struct MysqlGraphApp;

impl eframe::App for MysqlGraphApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
        eframe::egui::SidePanel::left("navigation")
            .resizable(false)
            .exact_width(200.0)
            .show(ctx, |ui| {
                ui.label("schema navigation area");
            });
        eframe::egui::TopBottomPanel::bottom("console")
            .resizable(false)
            .exact_height(200.0)
            .show(ctx, |ui| {
                ui.label("console output area");
            });
        // it also will be central panel rect
        let rest_rect = ctx.available_rect();
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_clip_rect(rest_rect);
            let mut table_node_1 = TableNode {
                id: 1,
                central_panel_rect: rest_rect,
                position: [50.0, 50.0].into(),
                scale: 1.0,
                columns: vec![
                    "id bigint".into(),
                    "name varchar(255)".into(),
                    "code varchar(255)".into(),
                    "pic_id bigint".into(),
                    "eng_type int".into(),
                    "deleted tinyint(1)".into(),
                ],
                name: "basic_engage".into(),
            };
            let mut table_node_2 = TableNode {
                id: 2,
                central_panel_rect: rest_rect,
                position: [350.0, 50.0].into(),
                scale: 1.0,
                columns: vec![
                    "id bigint".into(),
                    "gui varchar(64)".into(),
                    "gpn varchar(64)".into(),
                    "user_name varchar(64)".into(),
                    "user_type int".into(),
                    "deleted tinyint(1)".into(),
                ],
                name: "basic_staff".into(),
            };
            let join_point_1 = table_node_1.get_join_point_right(3);
            let join_point_2 = table_node_2.get_join_point_left(0);
            ui.painter().add(eframe::egui::Shape::CubicBezier(
                CubicBezierShape::from_points_stroke(
                    [
                        join_point_1,
                        Pos2::new(join_point_1.x + 15.0, join_point_1.y),
                        Pos2::new(join_point_2.x - 15.0, join_point_2.y),
                        join_point_2,
                    ],
                    false,
                    Color32::TRANSPARENT,
                    Stroke::new(2.0, Color32::from_rgb(255, 255, 255)),
                ),
            ));
            table_node_1.ui(ui);
            table_node_2.ui(ui);
        });
    }
}
