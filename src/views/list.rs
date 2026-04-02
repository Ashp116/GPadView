use egui::{self, Color32, CornerRadius, Stroke, Vec2};
use crate::controller_state::ControllerState;

pub fn show_list(ui: &mut egui::Ui, controllers: &[ControllerState]) -> Option<String> {
    let mut clicked = None;

    let total_width = ui.available_width();
    let side_padding = (total_width * 0.08).max(24.0).min(80.0);
    let inner_width = total_width - side_padding * 2.0;

    // Header
    ui.add_space(48.0);
    ui.vertical_centered(|ui| {
        ui.label(
            egui::RichText::new("CONTROLLERS")
                .size((inner_width * 0.07).clamp(24.0, 52.0))
                .color(Color32::from_rgb(0, 220, 180))
                .strong(),
        );
        ui.add_space(6.0);
        ui.label(
            egui::RichText::new("select a device to inspect")
                .size((inner_width * 0.025).clamp(11.0, 16.0))
                .color(Color32::from_rgb(80, 80, 110)),
        );
    });

    ui.add_space(40.0);

    // Grid config
    let gap = (inner_width * 0.03).clamp(10.0, 24.0);
    let min_cell = (inner_width * 0.25).clamp(180.0, 320.0);
    let max_cols = 4;
    let cols = ((inner_width + gap) / (min_cell + gap)).floor().max(1.0).min(max_cols as f32) as usize;
    let cell_width = (inner_width - gap * (cols as f32 - 1.0)) / cols as f32;
    let cell_height = (cell_width * 0.72).clamp(110.0, 200.0);
    let icon_radius = (cell_height * 0.18).clamp(16.0, 36.0);
    let name_size = (cell_width * 0.085).clamp(12.0, 18.0);
    let stat_size = (cell_width * 0.065).clamp(10.0, 14.0);

    // Side padding containers
    ui.horizontal(|ui| {
        ui.add_space(side_padding);

        ui.vertical(|ui| {
            egui::Grid::new("controller_grid")
                .num_columns(cols)
                .spacing(Vec2::new(gap, gap))
                .show(ui, |ui| {
                    for (i, controller) in controllers.iter().enumerate() {
                        let (rect, response) = ui.allocate_exact_size(
                            Vec2::new(cell_width, cell_height),
                            egui::Sense::click(),
                        );

                        let painter = ui.painter();

                        let bg_color = if response.hovered() {
                            Color32::from_rgb(26, 26, 40)
                        } else {
                            Color32::from_rgb(16, 16, 26)
                        };

                        painter.rect_filled(rect, CornerRadius::same(18), bg_color);
                        painter.rect_stroke(
                            rect,
                            CornerRadius::same(18),
                            Stroke::new(1.0, if response.hovered() {
                                Color32::from_rgb(0, 220, 180)
                            } else {
                                Color32::from_rgb(32, 32, 50)
                            }),
                            egui::StrokeKind::Inside,
                        );

                        // Top accent bar
                        let bar_width = cell_width * 0.25;
                        painter.rect_filled(
                            egui::Rect::from_min_size(
                                rect.min + Vec2::new((cell_width - bar_width) / 2.0, 0.0),
                                Vec2::new(bar_width, 3.0),
                            ),
                            CornerRadius::same(2),
                            Color32::from_rgb(0, 220, 180),
                        );

                        // Icon
                        let icon_pos = rect.min + Vec2::new(cell_width / 2.0, cell_height * 0.38);
                        painter.circle_filled(icon_pos, icon_radius, Color32::from_rgb(22, 22, 36));
                        painter.circle_stroke(
                            icon_pos,
                            icon_radius,
                            Stroke::new(1.5, Color32::from_rgb(0, 180, 140)),
                        );
                        painter.text(
                            icon_pos,
                            egui::Align2::CENTER_CENTER,
                            "⌨",
                            egui::FontId::proportional(icon_radius * 1.0),
                            Color32::from_rgb(0, 220, 180),
                        );

                        // Name
                        painter.text(
                            rect.min + Vec2::new(cell_width / 2.0, cell_height * 0.68),
                            egui::Align2::CENTER_CENTER,
                            &controller.name,
                            egui::FontId::proportional(name_size),
                            Color32::WHITE,
                        );

                        // Stats
                        painter.text(
                            rect.min + Vec2::new(cell_width / 2.0, cell_height * 0.84),
                            egui::Align2::CENTER_CENTER,
                            format!("{} buttons  •  {} axes", controller.buttons.len(), controller.axis.len()),
                            egui::FontId::proportional(stat_size),
                            Color32::from_rgb(70, 70, 100),
                        );

                        if response.clicked() {
                            clicked = Some(controller.id.clone());
                        }

                        if (i + 1) % cols == 0 {
                            ui.end_row();
                        }
                    }

                    if !controllers.is_empty() && controllers.len() % cols != 0 {
                        ui.end_row();
                    }
                });
        });

        ui.add_space(side_padding);
    });

    clicked
}