mod ControllerState;

use hidapi::HidApi;
use windows::Gaming::Input::RawGameController;

fn main() {
    let hid = HidApi::new().unwrap();


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

            let name = hid.device_list()
                .find(|d| d.vendor_id() == vid && d.product_id() == pid)
                .and_then(|d| d.product_string())
                .unwrap_or("Unknown");

            println!("=== {} ===", name);
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
    }
}