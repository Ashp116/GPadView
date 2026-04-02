use eframe::{egui, Frame};
use eframe::egui::Color32;
use egui::Ui;
use crate::controller_state::ControllerState;
use crate::views::{show_list, show_detail};

#[derive(Default)]
pub enum View {
    #[default]
    ControllerList,
    ControllerDetail(usize),
}

pub struct App {
    pub controllers: Vec<ControllerState>,
    pub view: View,
    initialized: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            controllers: vec![
                ControllerState::example(),
            ],
            view: View::default(),
            initialized: false,
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut Ui, frame: &mut Frame) {

    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.initialized {
            ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
            self.initialized = true;
        }
        ctx.set_visuals(egui::Visuals::dark());

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(Color32::from_rgb(12, 12, 18)))
            .show(ctx, |ui| {
                match &self.view {
                    View::ControllerList => {
                        if let Some(index) = show_list(ui, &self.controllers) {
                            self.view = View::ControllerDetail(index);
                        }
                    }
                    View::ControllerDetail(index) => {
                        let index = *index;
                        if show_detail(ui, &self.controllers[index]) {
                            self.view = View::ControllerList;
                        }
                    }
                }
            });
    }
}