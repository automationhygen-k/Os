#[derive(Clone)]
pub struct SystemSettings {
    pub accent_color: String,
    pub reduce_motion: bool,
    pub night_shift: bool,
    pub dock_magnification: bool,
    pub battery_saver: bool,
}

impl SystemSettings {
    pub fn new() -> Self {
        Self {
            accent_color: "Graphite".to_string(),
            reduce_motion: false,
            night_shift: false,
            dock_magnification: true,
            battery_saver: false,
        }
    }

    pub fn summary(&self) -> String {
        format!(
            "accent={} | reduce_motion={} | night_shift={} | dock_magnification={} | battery_saver={}",
            self.accent_color,
            self.reduce_motion,
            self.night_shift,
            self.dock_magnification,
            self.battery_saver
        )
    }

    pub fn set_accent(&mut self, color: &str) -> String {
        self.accent_color = color.to_string();
        format!("Accent color set to '{}'.", self.accent_color)
    }

    pub fn toggle_night_shift(&mut self) -> String {
        self.night_shift = !self.night_shift;
        format!("Night Shift set to {}.", self.night_shift)
    }

    pub fn set_battery_saver(&mut self, enabled: bool) -> String {
        self.battery_saver = enabled;
        format!("Battery Saver set to {}.", self.battery_saver)
    }
}
