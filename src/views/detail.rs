use egui::{self, Color32, CornerRadius, Stroke, Vec2};
use windows::Gaming::Input::GameControllerSwitchPosition;
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

    // after the buttons section, before the final request_repaint

    ui.add_space(32.0);

    if !controller.switches.is_empty() {
        // --- Switches label ---
        ui.vertical_centered(|ui| {
            ui.label(
                egui::RichText::new("SWITCHES")
                    .size(11.0)
                    .color(Color32::from_rgb(0, 220, 180))
                    .strong(),
            );
        });
        ui.add_space(10.0);

        // --- Switches centered ---
        let switch_size = btn_size * 1.2;
        let switch_gap = btn_gap;
        let per_row_s = ((inner_width + switch_gap) / (switch_size + switch_gap)).floor().max(1.0) as usize;
        let chunks_s: Vec<&[GameControllerSwitchPosition]> = controller.switches.chunks(per_row_s).collect();

        for row in &chunks_s {
            let row_width = row.len() as f32 * switch_size + (row.len() as f32 - 1.0) * switch_gap;
            let offset = ((total_width - row_width) / 2.0).max(0.0);

            ui.horizontal(|ui| {
                ui.add_space(offset);
                for (i, switch) in row.iter().enumerate() {
                    let (rect, _) = ui.allocate_exact_size(
                        Vec2::new(switch_size, switch_size),
                        egui::Sense::hover(),
                    );
                    let painter = ui.painter();
                    let center = rect.center();

                    // Background
                    painter.rect_filled(rect, CornerRadius::same(10), Color32::from_rgb(18, 18, 30));
                    painter.rect_stroke(
                        rect,
                        CornerRadius::same(10),
                        Stroke::new(1.0, Color32::from_rgb(35, 35, 55)),
                        egui::StrokeKind::Inside,
                    );

                    // Draw 8 direction dots
                    let dot_dist = switch_size * 0.28;
                    let dot_r = switch_size * 0.07;
                    let dirs: &[(f32, f32, GameControllerSwitchPosition)] = &[
                        ( 0.0, -1.0, GameControllerSwitchPosition::Up),
                        ( 1.0, -1.0, GameControllerSwitchPosition::UpRight),
                        ( 1.0,  0.0, GameControllerSwitchPosition::Right),
                        ( 1.0,  1.0, GameControllerSwitchPosition::DownRight),
                        ( 0.0,  1.0, GameControllerSwitchPosition::Down),
                        (-1.0,  1.0, GameControllerSwitchPosition::DownLeft),
                        (-1.0,  0.0, GameControllerSwitchPosition::Left),
                        (-1.0, -1.0, GameControllerSwitchPosition::UpLeft),
                    ];

                    for (dx, dy, dir) in dirs {
                        let pos = egui::pos2(
                            center.x + dx * dot_dist,
                            center.y + dy * dot_dist,
                        );
                        let active = switch == dir;
                        let color = if active {
                            Color32::from_rgb(0, 200, 160)
                        } else {
                            Color32::from_rgb(40, 40, 60)
                        };
                        painter.circle_filled(pos, dot_r, color);
                    }

                    // Center dot
                    let center_active = *switch == GameControllerSwitchPosition::Center;
                    painter.circle_filled(
                        center,
                        dot_r * 0.8,
                        if center_active {
                            Color32::from_rgb(60, 60, 80)
                        } else {
                            Color32::from_rgb(30, 30, 45)
                        },
                    );

                    // Label
                    painter.text(
                        egui::pos2(center.x, rect.max.y - 8.0),
                        egui::Align2::CENTER_CENTER,
                        format!("D{}", i + 1),
                        egui::FontId::proportional(switch_size * 0.18),
                        Color32::from_rgb(70, 70, 100),
                    );

                    if i < row.len() - 1 {
                        ui.add_space(switch_gap - item_spacing.x);
                    }
                }
            });
            ui.add_space(switch_gap - item_spacing.y);
        }
    }

    ui.ctx().request_repaint();
    go_back
}