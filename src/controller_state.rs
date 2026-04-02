use hidapi::HidApi;
use windows::Gaming::Input::{GameControllerSwitchPosition, RawGameController};

#[derive(Clone)]
pub struct ControllerState {
    pub name: String,
    pub buttons: Vec<bool>,
    pub axis: Vec<f64>,
    pub switches: Vec<GameControllerSwitchPosition>,
    pub raw_game_controller: Option<RawGameController>
}

impl ControllerState {
    pub fn example() -> Self {
        Self {
            name: "USB Gamepad".to_string(),
            buttons: vec![false, true, false, false, true, false, false, false, false, false, false, false],
            axis: vec![0.5, -0.3, 0.0, 0.8, 0.1, -1.0],
            switches: vec![
                GameControllerSwitchPosition::Center;
                0
            ],
            raw_game_controller: None,
        }
    }
    
    pub fn new(raw_game_controller: RawGameController, hid: &HidApi) -> Self {
        let vid = raw_game_controller.HardwareVendorId().unwrap();
        let pid = raw_game_controller.HardwareProductId().unwrap();

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
            raw_game_controller: Some(raw_game_controller)
        }
    }

    pub fn update(&mut self) {
        // self.raw_game_controller.unwrap().GetCurrentReading(
        //     self.buttons.as_mut_slice(),
        //     self.switches.as_mut_slice(),
        //     self.axis.as_mut_slice()
        // ).expect("Invalid Current Reading");
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_button_state(&self, index: usize) -> Option<&bool> {
        self.buttons.get(index)
    }

    pub fn get_axis_value(&self, index: usize) -> Option<&f64> {
        self.axis.get(index)
    }
}