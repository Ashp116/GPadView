use crate::app::App;
use crate::controller_manager::ControllerManager;

mod controller_state;
mod controller_manager;
mod views;
mod app;


fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([420.0, 600.0])
            .with_title("JoyView"),
        ..Default::default()
    };

    let controller_manager = ControllerManager::new();

    eframe::run_native("GPadView", options, Box::new(|_| Ok(Box::new(App::new(controller_manager))))).unwrap();
    // let controller_manager = ControllerManager::new();
    // loop {
    //     if (controller_manager.get_list().len() < 1) {
    //         continue
    //     }
    //
    //     let mut controller_state = controller_manager.update_controller_state_by_index(0).unwrap();
    //     print!("{:?}, {:?}, {:?}", controller_state.get_axis_value(0).unwrap(), controller_state.get_axis_value(1).unwrap(), controller_state.get_axis_value(2).unwrap());
    //     println!();
    // }

    /*let hid = HidApi::new().unwrap();


    loop {
        let controllers = RawGameController::RawGameControllers().unwrap();
        let count = controllers.Size().unwrap();

        for i in 0..count {
            let controller = controllers.GetAt(i).unwrap();

            let button_count = controller.ButtonCount().unwrap() as usize;
            let axis_count = controller.AxisCount().unwrap() as usize;
            let switch_count = controller.SwitchCount().unwrap() as usize;

            let vid = controller.HardwareVendorId().unwrap();
            let pid = controller.HardwareProductId().unwrap();

            // println!("  Buttons: {}", button_count);
            // println!("  Axes:    {}", axis_count);
            // println!("  Switches (POV hats): {}", switch_count);

            // Allocate buffers based on actual controller capabilities
            let mut buttons = vec![false; button_count];
            let mut axes = vec![0.0f64; axis_count];
            let mut switches = vec![
                windows::Gaming::Input::GameControllerSwitchPosition::Center;
                switch_count
            ];

            let _timestamp = controller
                .GetCurrentReading(
                    buttons.as_mut_slice(),
                    switches.as_mut_slice(),
                    axes.as_mut_slice(),
                )
                .unwrap();

            // for (i, &pressed) in buttons.iter().enumerate() {
            //     println!("  Button {}: {}", i, pressed);
            //     break;
            // }

            // for (i, &value) in axes.iter().enumerate() {
            //     println!("  Axis {}: {:.3}", i, value);
            // }
            //
            // for (i, switch) in switches.iter().enumerate() {
            //     println!("  Switch {}: {:?}", i, switch);
            // }
        }

        std::thread::sleep(std::time::Duration::from_millis(16));
    }*/
}