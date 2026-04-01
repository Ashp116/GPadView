pub struct ControllerState {
    name: Option<String>,
    buttons: Vec<bool>,
    axis: Vec<f64>
}

impl ControllerState {
    pub fn new(name: Option<String>, button_count: usize, axis_count: usize) -> Self {
        Self {
            name,
            buttons: vec![false; button_count],
            axis: vec![0.0; axis_count],
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