#[derive(Clone)]
pub struct Window {
    pub id: u32,
    pub title: String,
    pub workspace: String,
}

#[derive(Clone)]
pub struct DesktopShell {
    dock: Vec<String>,
    windows: Vec<Window>,
    workspace: String,
    next_window_id: u32,
    dock_magnification: f32,
    ui_smoothness_score: u8,
}

impl DesktopShell {
    pub fn new() -> Self {
        Self {
            dock: vec![
                "Finder".to_string(),
                "Terminal".to_string(),
                "Studio".to_string(),
                "Browser".to_string(),
                "All Apps".to_string(),
            ],
            windows: Vec::new(),
            workspace: "Main".to_string(),
            next_window_id: 1,
            dock_magnification: 1.08,
            ui_smoothness_score: 92,
        }
    }

    pub fn dock_summary(&self) -> String {
        format!(
            "Dock [{}] | magnification={:.2}x | smoothness={}%, motion=consistent",
            self.dock.join(" | "),
            self.dock_magnification,
            self.ui_smoothness_score
        )
    }

    pub fn pin_to_dock(&mut self, app_name: &str) -> String {
        if !self.dock.iter().any(|a| a.eq_ignore_ascii_case(app_name)) {
            self.dock.push(app_name.to_string());
        }
        self.dock_summary()
    }

    pub fn all_apps_page(&self, installed_apps: &[String]) -> String {
        if installed_apps.is_empty() {
            return "All Apps: no installed apps yet.".to_string();
        }

        let rows = installed_apps.join(" | ");
        format!(
            "All Apps Page -> {} | grid=8pt | sections=Dock Favorites + Installed",
            rows
        )
    }

    pub fn open_window(&mut self, title: &str) -> String {
        let id = self.next_window_id;
        self.next_window_id += 1;
        self.windows.push(Window {
            id,
            title: title.to_string(),
            workspace: self.workspace.clone(),
        });
        format!(
            "Opened window #{id} '{title}' on workspace '{}' with anchored transition.",
            self.workspace
        )
    }

    pub fn close_window(&mut self, id: u32) -> String {
        let before = self.windows.len();
        self.windows.retain(|w| w.id != id);
        if self.windows.len() == before {
            format!("No window found for id={id}.")
        } else {
            format!("Closed window id={id} with spatial return transition.")
        }
    }

    pub fn switch_workspace(&mut self, workspace: &str) -> String {
        self.workspace = workspace.to_string();
        format!(
            "Switched to workspace '{}' with state continuity.",
            self.workspace
        )
    }

    pub fn expose(&self) -> String {
        if self.windows.is_empty() {
            return "No active windows.".to_string();
        }

        let rows = self
            .windows
            .iter()
            .map(|w| format!("#{} '{}' [{}]", w.id, w.title, w.workspace))
            .collect::<Vec<_>>()
            .join(" | ");
        format!("Mission Control: {rows}")
    }
}
