use std::sync::{Arc, Mutex};
use eframe::{egui, Frame};
use eframe::egui::Color32;
use eframe::epaint::CornerRadius;
use egui::{Stroke, Ui, Vec2};
use crate::controller_manager::ControllerManager;
use crate::views::{show_list, show_detail, Toast};

#[derive(Default)]
pub enum View {
    #[default]
    ControllerList,
    ControllerDetail(String),
}

pub struct App {
    controller_manager: ControllerManager,
    view: View,
    initialized: bool,
    toasts: Arc<Mutex<Vec<Toast>>>,
}

impl App {
    pub fn new() -> Self {
        let toasts: Arc<Mutex<Vec<Toast>>> = Arc::new(Mutex::new(Vec::new()));
        let controller_manager = ControllerManager::new(Arc::clone(&toasts));

        Self {
            controller_manager,
            view: View::default(),
            initialized: false,
            toasts,
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, _ui: &mut Ui, _frame: &mut Frame) {

    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.initialized {
            ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
            self.initialized = true;
        }
        ctx.set_visuals(egui::Visuals::dark());

        let controllers = self.controller_manager.get_controllers_state();
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(Color32::from_rgb(12, 12, 18)))
            .show(ctx, |ui| {
                match &self.view {
                    View::ControllerList => {
                        if let Some(id) = show_list(ui, &controllers) {
                            self.view = View::ControllerDetail(id.clone());
                        }
                    }
                    View::ControllerDetail(id) => {
                        let id = id.clone();
                        self.controller_manager.update_controller_state(&id);
                        let state = self.controller_manager.get_controller(&id);
                        if show_detail(ui, state) {
                            self.view = View::ControllerList;
                        }
                    }
                }
            });

        // draw toasts
        let now = std::time::Instant::now();
        let mut toasts = self.toasts.lock().unwrap();

        toasts.retain(|t| now.duration_since(t.created_at) < t.duration);

        let toast_height = 64.0;
        let toast_width = 320.0;
        let toast_padding = 10.0;
        let screen_rect = ctx.screen_rect();

        for (i, toast) in toasts.iter().enumerate() {
            let elapsed = now.duration_since(toast.created_at).as_secs_f32();
            let alpha = if elapsed > toast.duration.as_secs_f32() - 0.5 {
                ((toast.duration.as_secs_f32() - elapsed) / 0.5).clamp(0.0, 1.0)
            } else {
                1.0
            };

            let x = screen_rect.max.x - toast_width - 16.0;
            let y = screen_rect.max.y - (toast_height + toast_padding) * (i as f32 + 1.0);
            let rect = egui::Rect::from_min_size(
                egui::pos2(x, y),
                Vec2::new(toast_width, toast_height),
            );

            let color_with_alpha = Color32::from_rgba_unmultiplied(
                toast.color.r(), toast.color.g(), toast.color.b(),
                (alpha * 255.0) as u8,
            );
            let bg = Color32::from_rgba_unmultiplied(16, 16, 26, (alpha * 240.0) as u8);

            let painter = ctx.layer_painter(egui::LayerId::new(
                egui::Order::Foreground,
                egui::Id::new(format!("toast_{}", i)),
            ));

            // Background
            painter.rect_filled(rect, CornerRadius::same(16), bg);
            painter.rect_stroke(
                rect,
                CornerRadius::same(16),
                Stroke::new(1.0, color_with_alpha),
                egui::StrokeKind::Inside,
            );

            // Top accent bar centered like list card
            // let bar_width = toast_width * 0.25;
            // painter.rect_filled(
            //     egui::Rect::from_min_size(
            //         rect.min + Vec2::new((toast_width - bar_width) / 2.0, 0.0),
            //         Vec2::new(bar_width, 3.0),
            //     ),
            //     CornerRadius::same(2),
            //     color_with_alpha,
            // );

            // Icon circle like list card
            let icon_pos = egui::pos2(rect.min.x + 44.0, rect.center().y);
            let icon_radius = 18.0;
            painter.circle_filled(icon_pos, icon_radius, Color32::from_rgba_unmultiplied(22, 22, 36, (alpha * 255.0) as u8));
            painter.circle_stroke(
                icon_pos,
                icon_radius,
                Stroke::new(1.5, color_with_alpha),
            );
            painter.text(
                icon_pos,
                egui::Align2::CENTER_CENTER,
                toast.icon,
                egui::FontId::proportional(16.0),
                color_with_alpha,
            );

            // Message and submessage
            let text_x = rect.min.x + 74.0;
            let max_chars = 28;
            let message = if toast.message.len() > max_chars {
                format!("{}...", &toast.message[..max_chars])
            } else {
                toast.message.clone()
            };

            painter.text(
                egui::pos2(text_x, rect.center().y),
                egui::Align2::LEFT_CENTER,
                &message,
                egui::FontId::proportional(14.0),
                Color32::from_rgba_unmultiplied(255, 255, 255, (alpha * 255.0) as u8),
            );
        }

        ctx.request_repaint_after(std::time::Duration::from_millis(16));
    }
}