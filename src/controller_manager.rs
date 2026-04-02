use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use hidapi::HidApi;
use windows::Foundation::{EventHandler};
use windows::Gaming::Input::RawGameController;
use crate::controller_state::ControllerState;
use crate::views::Toast;

pub struct ControllerManager {
    game_controllers: Arc<Mutex<HashMap<String, ControllerState>>>,
    token_added: Option<i64>,
    token_removed: Option<i64>,
}

impl ControllerManager {
    pub fn new(toasts: Arc<Mutex<Vec<Toast>>>) -> Self {
        let game_controllers: Arc<Mutex<HashMap<String, ControllerState>>> = Arc::new(Mutex::new(HashMap::new()));
        let hid = Arc::new(Mutex::new(HidApi::new().unwrap()));

        // Get all the connected controllers
        let controllers = RawGameController::RawGameControllers().unwrap();
        let controller_count = controllers.Size().unwrap();

        for i in 0..controller_count {
            let controller = controllers.GetAt(i).unwrap();
            let mut hid_guard = hid.lock().unwrap();
            hid_guard.refresh_devices().unwrap();
            game_controllers.lock().unwrap().insert(
                controller.NonRoamableId().unwrap().to_string(),
                ControllerState::new(controller, &mut hid_guard)
            );
        }

        let controllers_clone = Arc::clone(&game_controllers);
        let hid_clone = Arc::clone(&hid);
        let toasts_clone = Arc::clone(&toasts);

        let token_added = RawGameController::RawGameControllerAdded(
            &EventHandler::<RawGameController>::new(move |_, controller| {
                let c = controller.unwrap();
                let id = c.NonRoamableId().unwrap().to_string();
                let mut hid_guard = hid_clone.lock().unwrap();
                hid_guard.refresh_devices().unwrap();
                let state = ControllerState::new(c.clone(), &mut hid_guard);
                let name = state.name.clone();
                controllers_clone.lock().unwrap().insert(id, state);
                toasts_clone.lock().unwrap().push(Toast::connected(&name));
                Ok(())
            })
        ).unwrap();

        let controllers_clone2 = Arc::clone(&game_controllers);
        let toasts_clone2 = Arc::clone(&toasts);

        let token_removed = RawGameController::RawGameControllerRemoved(
            &EventHandler::<RawGameController>::new(move |_, controller| {
                let c = controller.unwrap();
                let id = c.NonRoamableId().unwrap().to_string();
                let mut guard = controllers_clone2.lock().unwrap();
                let name = guard.get(&id)
                    .map(|s| s.name.clone())
                    .unwrap_or_else(|| "Controller".to_string());
                guard.remove(&id);
                toasts_clone2.lock().unwrap().push(Toast::disconnected(&name));
                Ok(())
            })
        ).unwrap();

        Self {
            game_controllers,
            token_added: Some(token_added),
            token_removed: Some(token_removed),
        }
    }

    pub fn get_id_list(&self) -> Vec<String> {
        self.game_controllers.lock().unwrap().keys().cloned().collect()
    }

    pub fn get_controllers_state(&self) -> Vec<ControllerState> {
        self.game_controllers.lock().unwrap().values().cloned().collect()
    }

    pub fn get_controller(&self, id: &str) -> Option<ControllerState> {
        self.game_controllers.lock().unwrap().get(id).cloned()
    }

    pub fn is_controller_connected(&self, id: String) -> bool {
        self.game_controllers.lock().unwrap().contains_key(id.as_str()).clone()
    }

    pub fn update_controller_state_by_index(&self, index: usize) -> Option<ControllerState> {
        let id = self.get_id_list().get(index)?.clone();
        self.update_controller_state(&*id)
    }

    pub fn update_controller_state(&self, id: &str) -> Option<ControllerState> {
        let mut guard = self.game_controllers.lock().unwrap();
        let state = guard.get_mut(id)?;

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