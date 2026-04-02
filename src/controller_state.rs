use hidapi::HidApi;
use windows::Gaming::Input::{GameControllerSwitchPosition, RawGameController};

#[derive(Clone)]
pub struct ControllerState {
    pub name: String,
    pub buttons: Vec<bool>,
    pub axis: Vec<f64>,
    switches: Vec<GameControllerSwitchPosition>,
    raw_game_controller: RawGameController
}

impl ControllerState {

    pub fn new(raw_game_controller: RawGameController, hid: &mut HidApi) -> Self {
        let vid = raw_game_controller.HardwareVendorId().unwrap();
        let pid = raw_game_controller.HardwareProductId().unwrap();

        hid.refresh_devices().unwrap();
        let name = hid.device_list()
            .find(|d| d.vendor_id() == vid && d.product_id() == pid)
            .and_then(|d| d.product_string())
            .unwrap_or("Unknown");

        let button_count = raw_game_controller.ButtonCount().unwrap() as usize;
        let axis_count = raw_game_controller.AxisCount().unwrap() as usize;
        let switch_count = raw_game_controller.SwitchCount().unwrap() as usize;

        let buttons = vec![false; button_count];
        let axis = vec![0.0f64; axis_count];
        let switches = vec![
            GameControllerSwitchPosition::Center;
            switch_count
        ];

        Self {
            name: String::from(name),
            buttons,
            axis,
            switches,
            raw_game_controller
        }
    }

    pub fn update(&mut self) {
        self.raw_game_controller.GetCurrentReading(
            self.buttons.as_mut_slice(),
            self.switches.as_mut_slice(),
            self.axis.as_mut_slice()
        ).expect("Invalid Current Reading");
    }
    
    pub fn get_button_state(&self, index: usize) -> Option<&bool> {
        self.buttons.get(index)
    }

    pub fn get_axis_value(&self, index: usize) -> Option<&f64> {
        self.axis.get(index)
    }
}