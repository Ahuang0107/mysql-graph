// pub struct UiState {
//     canvas_scale: SliderValue,
// }
//
// struct SliderValue {
//     value: f32,
//     max: f32,
//     min: f32,
// }
//
// impl SliderValue {
//     pub fn increase(&mut self, value: f32) {
//         self.value += value;
//         if self.value < self.min {
//             self.value = self.min;
//         } else if self.value > self.max {
//             self.value = self.max
//         }
//     }
// }
//
// impl UiState {
//     fn update(&mut self, ctx: &eframe::egui::Context) {
//         ctx.input(|input_state| {
//             if input_state.zoom_delta() > 1.0 {
//                 self.canvas_scale.increase(0.02);
//             } else if input_state.zoom_delta() < 1.0 {
//                 self.canvas_scale.increase(-0.02);
//             }
//             for x in input_state.events.iter() {
//                 match x {
//                     Event::PointerButton {
//                         pos,
//                         button,
//                         pressed,
//                         ..
//                     } => match button {
//                         PointerButton::Primary => if *pressed {},
//                         PointerButton::Secondary => if *pressed {},
//                         _ => {}
//                     },
//                     _ => {}
//                 }
//             }
//         });
//     }
// }
