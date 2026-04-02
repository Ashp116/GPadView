use eframe::{egui, Frame};
use eframe::egui::Color32;
use egui::Ui;
use crate::controller_manager::ControllerManager;
use crate::views::{show_list, show_detail};

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
}

impl App {
    pub fn new(controller_manager: ControllerManager) -> Self {
        Self {
            controller_manager,
            view: View::default(),
            initialized: false,
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
            .frame(egui::Frame::none().fill(Color32::from_rgb(12, 12, 18)))
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

        ctx.request_repaint_after(std::time::Duration::from_millis(16));
    }
}