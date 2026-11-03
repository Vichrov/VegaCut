//! VegaCut wordmark, drawn with the bundled open UI font.

/// Draw the fork name at the requested position without embedding upstream trademarks.
pub fn paint_wordmark(ui: &egui::Ui, left_center: egui::Pos2, height: f32, dark: bool) -> Option<egui::Rect> {
    let color = if dark { egui::Color32::from_rgb(73, 184, 209) } else { egui::Color32::from_rgb(22, 103, 130) };
    Some(ui.painter().text(left_center, egui::Align2::LEFT_CENTER, "VegaCut", crate::theme::Tokens::semibold(height), color))
}
