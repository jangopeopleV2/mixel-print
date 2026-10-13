//! Original MIXEL monogram, drawn in code under MIT OR Apache-2.0.

use egui::{Color32, Rect, Response, Sense, Stroke, pos2, vec2};

/// The original MIXEL M, sized to a square and labelled for assistive technologies.
pub(crate) fn mark(ui: &mut egui::Ui, size: f32) -> Response {
    let (rect, response) = ui.allocate_exact_size(vec2(size, size), Sense::hover());
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Other, ui.is_enabled(), "MIXEL"));
    let side = rect.width().min(rect.height());
    let area = Rect::from_center_size(rect.center(), vec2(side, side));
    let point = |x: f32, y: f32| pos2(area.left() + side * x, area.top() + side * y);
    let vertices = [point(0.18, 0.82), point(0.18, 0.18), point(0.5, 0.53), point(0.82, 0.18), point(0.82, 0.82)];
    let stroke = Stroke::new(side * 0.11, Color32::from_rgb(0x21, 0xb6, 0xa8));
    for pair in vertices.windows(2) {
        ui.painter().line_segment([pair[0], pair[1]], stroke);
    }
    response
}
