use eframe::egui::{Align2, Color32, FontId, Pos2, Rect, Rounding, Sense, Shape, Stroke, Vec2};
use eframe::egui::{Response, Ui, Widget};
use eframe::epaint::{CircleShape, FontFamily, RectShape};

use crate::data_source::TableColumn;

const WIDTH: f32 = 236.0;
const BACKGROUND_COLOR: Color32 = Color32::from_rgb(48, 48, 48);
const STROKE_COLOR: Color32 = Color32::from_rgb(25, 25, 25);
const STROKE_HOVER_COLOR: Color32 = Color32::WHITE;

const HEADER_HEIGHT: f32 = 35.0;
const HEADER_COLOR: Color32 = Color32::from_rgb(60, 29, 39);
const HEADER_STROKE_BOTTOM_COLOR: Color32 = Color32::from_rgba_premultiplied(36, 36, 36, 255);

const JOIN_POINT_SPACE: f32 = 35.0;

#[derive(Debug, Clone)]
pub struct TableNode {
    pub position: Pos2,
    pub scale: f32,
    pub columns: Vec<TableColumn>,
    pub name: String,
}

impl TableNode {
    pub fn new(name: &str, columns: Vec<TableColumn>) -> Self {
        Self {
            position: Pos2::default(),
            scale: 1.0,
            columns,
            name: name.to_string(),
        }
    }
}

impl Widget for &mut TableNode {
    fn ui(self, ui: &mut Ui) -> Response {
        let scale = self.scale;
        let position = Pos2::new(self.position.x * scale, self.position.y * scale);
        let size = Vec2::new(
            WIDTH,
            HEADER_HEIGHT + JOIN_POINT_SPACE * (self.columns.len() + 1) as f32,
        ) * scale;

        let header_size = Vec2::new(size.x, HEADER_HEIGHT * scale);
        let header_rounding = Rounding {
            nw: 7.0 * scale,
            ne: 7.0 * scale,
            sw: 0.0 * scale,
            se: 0.0 * scale,
        };
        let header_stroke = 1.5 * scale;

        let body_rounding = Rounding::same(7.0 * scale);
        let body_stroke = 1.5 * scale;

        let body_area = Rect::from_min_size(position, size);
        let body = Shape::Rect(RectShape {
            rect: body_area,
            rounding: body_rounding,
            fill: BACKGROUND_COLOR,
            stroke: Stroke::NONE,
        });
        ui.painter().add(body);
        let header = Shape::Rect(RectShape {
            rect: Rect::from_min_size(position, header_size),
            rounding: header_rounding,
            fill: HEADER_COLOR,
            stroke: Stroke::new(header_stroke, HEADER_STROKE_BOTTOM_COLOR),
        });
        ui.painter().add(header);
        let hover_pos = ui.ctx().pointer_hover_pos();
        let mut outline_color = STROKE_COLOR;
        if let Some(hover_pos) = hover_pos {
            if body_area.contains(hover_pos) {
                outline_color = STROKE_HOVER_COLOR
            }
        }
        let body_stroke = Shape::Rect(RectShape {
            rect: body_area,
            rounding: body_rounding,
            fill: Color32::TRANSPARENT,
            stroke: Stroke::new(body_stroke, outline_color),
        });
        ui.painter().add(body_stroke);
        let title_font = FontId::new(18.0 * scale, FontFamily::Proportional);
        let title_position = Pos2::new(position.x + 20.0 * scale, position.y + header_size.y / 2.0);
        ui.painter().text(
            title_position,
            Align2::LEFT_CENTER,
            &self.name,
            title_font.clone(),
            Color32::WHITE,
        );
        let join_point_init_pos = position + Vec2::new(0.0, 60.0) * scale;
        let join_point_space = Vec2::new(0.0, JOIN_POINT_SPACE) * scale;
        let join_point_radius = 9.0 * scale;
        let join_point_stroke = 1.5 * scale;
        for (index, column) in self.columns.iter().enumerate() {
            let join_point_pos = join_point_init_pos + join_point_space * (index as f32);
            let join_point = Shape::Circle(CircleShape {
                center: join_point_pos,
                radius: join_point_radius,
                fill: Color32::from_rgb(99, 198, 99),
                stroke: Stroke::new(join_point_stroke, Color32::from_rgb(25, 25, 25)),
            });
            ui.painter().add(join_point);
            let join_point_label_pos = join_point_pos + Vec2::new(20.0, 0.0) * scale;
            ui.painter().text(
                join_point_label_pos,
                Align2::LEFT_CENTER,
                column.field.as_str(),
                title_font.clone(),
                Color32::WHITE,
            );
        }
        let response = ui.interact(Rect::from_min_size(position, size), ui.id(), Sense::drag());
        if response.dragged() {
            let delta = response.drag_delta();
            self.position += delta / scale;
        }
        response
    }
}
