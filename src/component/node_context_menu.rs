use eframe::egui::{
    Align2, Event, FontFamily, FontId, PointerButton, Pos2, Response, Sense, Shape, Stroke, Ui,
    Vec2, Widget,
};
use eframe::emath::Rect;
use eframe::epaint::{Color32, RectShape, Rounding};

/*整个菜单样式*/
const WIDTH: f32 = 233.0;
const BACKGROUND_COLOR: Color32 = Color32::from_rgb(24, 24, 24);
const STROKE_WIDTH: f32 = 1.0;
const STROKE_COLOR: Color32 = Color32::from_rgb(47, 47, 47);

/*菜单标题栏样式*/
const HEADER_HEIGHT: f32 = 32.0;
const HEADER_ROUNDING: Rounding = Rounding {
    nw: 5.0,
    ne: 5.0,
    sw: 0.0,
    se: 0.0,
};
const HEADER_FONT: FontId = FontId::new(12.0, FontFamily::Proportional);
const HEADER_FONT_COLOR: Color32 = Color32::from_rgb(152, 152, 152);

/*菜单元素样式*/
const ITEM_HEIGHT: f32 = 18.0;
const ITEM_WIDTH: f32 = 224.0;
const ITEM_SPACE_HEIGHT: f32 = 4.0;
const ITEM_HOVER_COLOR: Color32 = Color32::from_rgb(71, 114, 179);
const ITEM_FONT: FontId = FontId::new(12.0, FontFamily::Proportional);
const ITEM_FONT_COLOR: Color32 = Color32::WHITE;

#[derive(Debug, Clone)]
pub struct NodeContextMenu {
    pos: Pos2,
    rect: Rect,
    item_list: Vec<(String, Option<Rect>)>,
}

impl NodeContextMenu {
    pub fn update_pos(&mut self, pos: Pos2) {
        self.pos = pos;
    }
    pub fn if_clicked(&self, ui: &Ui) -> Option<String> {
        let mut clicked_pos = None;
        ui.ctx().input(|input_state| {
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
                                clicked_pos = Some(*pos)
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        });
        if let Some(clicked_pos) = clicked_pos {
            for (item, pos) in self.item_list.iter() {
                if let Some(pos) = pos {
                    if pos.contains(clicked_pos) {
                        return Some(item.clone());
                    }
                }
            }
        }
        None
    }
    pub fn contains(&self, pos: Pos2) -> bool {
        self.rect.contains(pos)
    }
}

impl Default for NodeContextMenu {
    fn default() -> Self {
        Self {
            pos: Pos2::default(),
            rect: Rect::NOTHING,
            item_list: vec![
                (String::from("Table Node"), None),
                (String::from("Math Node"), None),
            ],
        }
    }
}

impl Widget for &mut NodeContextMenu {
    fn ui(self, ui: &mut Ui) -> Response {
        // 绘制整个背景
        let response = {
            let item_count = self.item_list.len();
            let ui_area = Rect::from_min_size(
                self.pos,
                Vec2::new(
                    WIDTH,
                    HEADER_HEIGHT
                        + ITEM_HEIGHT * item_count as f32
                        + ITEM_SPACE_HEIGHT * (item_count + 1) as f32,
                ),
            );
            self.rect = ui_area;
            ui.painter().add(Shape::Rect(RectShape {
                rect: ui_area,
                rounding: Rounding::same(5.0),
                fill: BACKGROUND_COLOR,
                stroke: Stroke::new(STROKE_WIDTH, STROKE_COLOR),
            }));
            ui.interact(ui_area, ui.id(), Sense::hover())
        };

        // 绘制标题栏
        {
            let header_size = Vec2::new(WIDTH, HEADER_HEIGHT);
            let header = Shape::Rect(RectShape {
                rect: Rect::from_min_size(self.pos, header_size),
                rounding: HEADER_ROUNDING,
                fill: BACKGROUND_COLOR,
                stroke: Stroke::new(STROKE_WIDTH, STROKE_COLOR),
            });
            ui.painter().add(header);
            let title_position = Pos2::new(self.pos.x + 10.0, self.pos.y + HEADER_HEIGHT / 2.0);
            ui.painter().text(
                title_position,
                Align2::LEFT_CENTER,
                "Node Context Menu",
                HEADER_FONT,
                HEADER_FONT_COLOR,
            );
        }

        // 绘制可选项
        {
            let hover_pos = ui.ctx().pointer_hover_pos();
            for (index, (item_name, click_area)) in self.item_list.iter_mut().enumerate() {
                let item_area = Rect::from_min_size(
                    self.pos
                        + Vec2::new((WIDTH - ITEM_WIDTH) / 2.0, HEADER_HEIGHT)
                        + Vec2::new(0.0, ITEM_SPACE_HEIGHT)
                        + Vec2::new(0.0, ITEM_HEIGHT * index as f32),
                    Vec2::new(ITEM_WIDTH, ITEM_HEIGHT),
                );
                let mut fill_color = BACKGROUND_COLOR;
                if let Some(hover_pos) = hover_pos {
                    if item_area.contains(hover_pos) {
                        fill_color = ITEM_HOVER_COLOR
                    }
                }
                let item = Shape::Rect(RectShape {
                    rect: item_area,
                    rounding: Rounding::same(2.0),
                    fill: fill_color,
                    stroke: Stroke::NONE,
                });
                ui.painter().add(item);
                let item_title_position =
                    Pos2::new(item_area.min.x + 12.0, item_area.min.y + ITEM_HEIGHT / 2.0);
                ui.painter().text(
                    item_title_position,
                    Align2::LEFT_CENTER,
                    item_name,
                    ITEM_FONT,
                    ITEM_FONT_COLOR,
                );
                *click_area = Some(item_area);
            }
        }

        response
    }
}
