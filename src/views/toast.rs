use egui::Color32;

#[derive(Clone)]
pub struct Toast {
    pub message: String,
    pub color: Color32,
    pub icon: char,
    pub created_at: std::time::Instant,
    pub duration: std::time::Duration,
}

impl Toast {
    pub fn connected(name: &str) -> Self {
        Self {
            message: format!("{} Connected!", name),
            color: Color32::from_rgb(0, 200, 160),
            icon: '⌨',
            created_at: std::time::Instant::now(),
            duration: std::time::Duration::from_secs(3),
        }
    }

    pub fn disconnected(name: &str) -> Self {
        Self {
            message: format!("{} Disconnected!", name),
            color: Color32::from_rgb(180, 80, 80),
            icon: '⌨',
            created_at: std::time::Instant::now(),
            duration: std::time::Duration::from_secs(3),
        }
    }
}