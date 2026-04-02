use egui::{self, Color32, CornerRadius, Stroke, Vec2};
use crate::controller_state::ControllerState;

pub fn show_detail(ui: &mut egui::Ui, controller: Option<ControllerState>) -> bool {
    let controller = match controller {
        Some(c) => c,
        None => return true,
    };

    let mut go_back = false;

    let total_width = ui.available_width();
    let side_padding = (total_width * 0.08).max(24.0).min(80.0);
    let inner_width = total_width - side_padding * 2.0;

    let btn_size = (inner_width * 0.07).clamp(36.0, 60.0);
    let btn_gap = (inner_width * 0.015).clamp(6.0, 14.0);
    let per_row = ((inner_width + btn_gap) / (btn_size + btn_gap)).floor().max(1.0) as usize;

    let slider_width = (inner_width * 0.3).clamp(100.0, 260.0);
    let axis_count = controller.axis.len();
    let axes_cols = 2_usize;
    let axis_col_width = 70.0 + slider_width + 50.0 + 16.0;
    let axes_grid_width = axis_col_width * axes_cols as f32 + 24.0;
    let axes_offset = ((total_width - axes_grid_width) / 2.0).max(0.0);

    ui.add_space(24.0);

    // --- Back button ---
    ui.horizontal(|ui| {
        ui.add_space(side_padding);
        let back_response = ui.add(
            egui::Button::new(
                egui::RichText::new("Back")
                    .color(Color32::from_rgb(0, 220, 180))
                    .size(24.0),
            )
                .fill(Color32::from_rgb(18, 18, 28))
                .stroke(Stroke::new(1.0, Color32::from_rgb(0, 180, 140)))
                .corner_radius(CornerRadius::same(8))
                .frame(false),
        );
        if back_response.clicked() {
            go_back = true;
        }
    });

    ui.add_space(16.0);

    // --- Centered title ---
    ui.vertical_centered(|ui| {
        ui.label(
            egui::RichText::new(&controller.name)
                .size((inner_width * 0.05).clamp(20.0, 36.0))
                .color(Color32::WHITE)
                .strong(),
        );
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new(format!(
                "{} buttons  •  {} axes",
                controller.buttons.len(),
                controller.axis.len()
            ))
                .size(12.0)
                .color(Color32::from_rgb(80, 80, 110)),
        );
    });

    ui.add_space(32.0);

    // --- Axes label ---
    ui.vertical_centered(|ui| {
        ui.label(
            egui::RichText::new("AXES")
                .size(11.0)
                .color(Color32::from_rgb(0, 220, 180))
                .strong(),
        );
    });
    ui.add_space(10.0);

    // --- Axes grid ---
    ui.horizontal(|ui| {
        ui.add_space(axes_offset);
        egui::Grid::new("axes_grid")
            .num_columns(axes_cols)
            .spacing(Vec2::new(24.0, 12.0))
            .show(ui, |ui| {
                for (i, &value) in controller.axis.iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(format!("Axis {:>2}", i))
                                .size(12.0)
                                .color(Color32::from_rgb(120, 120, 150))
                                .monospace(),
                        );
                        ui.add_space(8.0);

                        let (rect, _) = ui.allocate_exact_size(
                            Vec2::new(slider_width, 14.0),
                            egui::Sense::hover(),
                        );
                        let painter = ui.painter();

                        painter.rect_filled(rect, CornerRadius::same(4), Color32::from_rgb(20, 20, 32));

                        let fill_width = (value as f32).clamp(0.0, 1.0) * (rect.width());
                        let fill_rect = egui::Rect::from_min_max(
                                egui::pos2(rect.min.x, rect.min.y + 2.0),
                                egui::pos2((rect.min.x + fill_width).min(rect.max.x), rect.max.y - 2.0),
                        );

                        painter.rect_filled(fill_rect, CornerRadius::same(3), Color32::from_rgb(0, 200, 160));

                        ui.add_space(8.0);
                        ui.label(
                            egui::RichText::new(format!("{:+.2}", value))
                                .size(12.0)
                                .color(Color32::from_rgb(0, 220, 180))
                                .monospace(),
                        );
                    });

                    if i % 2 == 1 {
                        ui.end_row();
                    }
                }
                if axis_count % 2 != 0 {
                    ui.end_row();
                }
            });
    });

    ui.add_space(32.0);

    // --- Buttons label ---
    ui.vertical_centered(|ui| {
        ui.label(
            egui::RichText::new("BUTTONS")
                .size(11.0)
                .color(Color32::from_rgb(0, 220, 180))
                .strong(),
        );
    });
    ui.add_space(10.0);

    // --- Buttons rows manually centered ---
    let item_spacing = ui.spacing().item_spacing;
    let chunks: Vec<&[bool]> = controller.buttons.chunks(per_row).collect();
    for row in &chunks {
        let row_width = row.len() as f32 * btn_size + (row.len() as f32 - 1.0) * btn_gap;
        let offset = ((total_width - row_width) / 2.0).max(0.0);

        ui.horizontal(|ui| {
            ui.add_space(offset);
            for (i, &pressed) in row.iter().enumerate() {
                let (bg, text_color) = if pressed {
                    (Color32::from_rgb(0, 200, 160), Color32::from_rgb(10, 10, 20))
                } else {
                    (Color32::from_rgb(18, 18, 30), Color32::from_rgb(70, 70, 100))
                };

                let (rect, _) = ui.allocate_exact_size(
                    Vec2::new(btn_size, btn_size),
                    egui::Sense::hover(),
                );
                let painter = ui.painter();

                painter.rect_filled(rect, CornerRadius::same(10), bg);
                if !pressed {
                    painter.rect_stroke(
                        rect,
                        CornerRadius::same(10),
                        Stroke::new(1.0, Color32::from_rgb(35, 35, 55)),
                        egui::StrokeKind::Inside,
                    );
                }
                painter.text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    format!("{}", i + 1),
                    egui::FontId::proportional(btn_size * 0.3),
                    text_color,
                );

                if i < row.len() - 1 {
                    ui.add_space(btn_gap - item_spacing.x);
                }
            }
        });

        ui.add_space(btn_gap - item_spacing.y);
    }

    ui.ctx().request_repaint();
    go_back
}