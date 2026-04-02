use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use hidapi::HidApi;
use windows::Foundation::{EventHandler};
use windows::Gaming::Input::RawGameController;
use crate::controller_state::ControllerState;

pub struct ControllerManager {
    game_controllers: Arc<Mutex<HashMap<String, ControllerState>>>,
    token_added: Option<i64>,
    token_removed: Option<i64>,
}

impl ControllerManager {
    pub fn new() -> Self{
        let game_controllers: Arc<Mutex<HashMap<String, ControllerState>>> = Arc::new(Mutex::new(HashMap::new()));
        let hid = Arc::new(Mutex::new(HidApi::new().unwrap()));

        // Get all the connected controllers
        let controllers = RawGameController::RawGameControllers().unwrap();
        let controller_count = controllers.Size().unwrap();

        for i in 0..controller_count {
            let controller = controllers.GetAt(i).unwrap();
            let mut hid_guard = hid.lock().unwrap();
            hid_guard.refresh_devices().unwrap();
            game_controllers.lock().unwrap().insert(controller.NonRoamableId().unwrap().to_string(), ControllerState::new(controller, &mut hid_guard));
        }

        // Events for changes in connected controllers
        let controllers_clone = Arc::clone(&game_controllers);
        let hid_clone = Arc::clone(&hid);

        let token_added = RawGameController::RawGameControllerAdded(
            &EventHandler::<RawGameController>::new(move |_, controller| {
                let c = controller.unwrap();
                let id = c.NonRoamableId().unwrap().to_string();
                let mut hid_guard = hid_clone.lock().unwrap();
                hid_guard.refresh_devices().unwrap();
                let state = ControllerState::new(c.clone(), &mut hid_guard);
                controllers_clone.lock().unwrap().insert(id, state);
                Ok(())
            })
        ).unwrap();

        let controllers_clone2 = Arc::clone(&game_controllers);

        let token_removed = RawGameController::RawGameControllerRemoved(
            &EventHandler::<RawGameController>::new(move |_, controller| {
                let c = controller.unwrap();
                let id = c.NonRoamableId().unwrap().to_string();
                controllers_clone2.lock().unwrap().remove(&id);
                Ok(())
            })
        ).unwrap();

        Self {
            game_controllers,
            token_added: Some(token_added),
            token_removed: Some(token_removed),
        }
    }

    pub fn get_list(&self) -> HashMap<String, ControllerState> {
        self.game_controllers.lock().unwrap().clone()
    }

    pub fn get_id_list(&self) -> Vec<String> {
        self.game_controllers.lock().unwrap().keys().cloned().collect()
    }

    pub fn get_controllers_state(&self) -> Vec<ControllerState> {
        self.game_controllers.lock().unwrap().values().cloned().collect()
    }

    pub fn get_controller(&self, id: String) -> Option<ControllerState> {
        self.game_controllers.lock().unwrap().get(id.as_str()).cloned()
    }

    pub fn is_controller_connected(&self, id: String) -> bool {
        self.game_controllers.lock().unwrap().contains_key(id.as_str()).clone()
    }

    pub fn update_controller_state_by_index(&self, index: usize) -> Option<ControllerState> {
        let id = self.get_id_list().get(index)?.clone();
        self.update_controller_state(id)
    }

    pub fn update_controller_state(&self, id: String) -> Option<ControllerState> {
        let mut guard = self.game_controllers.lock().unwrap();
        let state = guard.get_mut(id.as_str())?;

        state.update();
        Some(state.clone())
    }
}

impl Drop for ControllerManager {
    fn drop(&mut self) {
        if let Some(token) = self.token_added.take() {
            RawGameController::RemoveRawGameControllerAdded(token).unwrap();
        }

        if let Some(token) = self.token_removed.take() {
            RawGameController::RemoveRawGameControllerRemoved(token).unwrap();
        }
    }
}