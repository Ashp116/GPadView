use hidapi::HidApi;
use windows::Gaming::Input::RawGameController;

#[derive(Clone)]
pub struct ControllerState {
    name: String,
    buttons: Vec<bool>,
    axis: Vec<f64>,
    raw_game_controller: RawGameController
}

impl ControllerState {
    pub fn new(raw_game_controller: RawGameController, hid: &HidApi) -> Self {
        let vid = raw_game_controller.HardwareVendorId().unwrap();
        let pid = raw_game_controller.HardwareProductId().unwrap();

        let name = hid.device_list()
            .find(|d| d.vendor_id() == vid && d.product_id() == pid)
            .and_then(|d| d.product_string())
            .unwrap_or("Unknown");
        
        let button_count = raw_game_controller.ButtonCount().unwrap() as usize;
        let axis_count = raw_game_controller.AxisCount().unwrap() as usize;
        
        Self {
            name: String::from(name),
            buttons: vec![false; button_count],
            axis: vec![0.0; axis_count],
            raw_game_controller
        }
    }
    
    pub fn update_axis(mut self, index: usize, value: f64) {
        if let Some(axis) = self.axis.get_mut(index) {
            *axis = value;
        } else {
            println!("Axis index out of bounds!")
        }
    }

    pub fn update_button(mut self, index: usize, value: bool) {
        if let Some(axis) = self.buttons.get_mut(index) {
            *axis = value;
        } else {
            println!("Button index out of bounds!")
        }
    }
}