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
}

impl DesktopShell {
    pub fn new() -> Self {
        Self {
            dock: vec![
                "Finder".to_string(),
                "Terminal".to_string(),
                "Studio".to_string(),
                "Browser".to_string(),
            ],
            windows: Vec::new(),
            workspace: "Main".to_string(),
            next_window_id: 1,
        }
    }

    pub fn dock_summary(&self) -> String {
        format!("Dock [{}]", self.dock.join(" | "))
    }

    pub fn pin_to_dock(&mut self, app_name: &str) -> String {
        if !self.dock.iter().any(|a| a.eq_ignore_ascii_case(app_name)) {
            self.dock.push(app_name.to_string());
        }
        self.dock_summary()
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
            "Opened window #{id} '{title}' on workspace '{}'.",
            self.workspace
        )
    }

    pub fn close_window(&mut self, id: u32) -> String {
        let before = self.windows.len();
        self.windows.retain(|w| w.id != id);
        if self.windows.len() == before {
            format!("No window found for id={id}.")
        } else {
            format!("Closed window id={id}.")
        }
    }

    pub fn switch_workspace(&mut self, workspace: &str) -> String {
        self.workspace = workspace.to_string();
        format!("Switched to workspace '{}'.", self.workspace)
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
