#[derive(Clone, Debug)]
pub enum FocusMode {
    Flow,
    DeepWork,
    Meeting,
    Recovery,
}

#[derive(Clone)]
pub struct Workbench {
    current_focus: FocusMode,
    workspace_name: String,
    pinned_actions: Vec<String>,
}

impl Workbench {
    pub fn new() -> Self {
        Self {
            current_focus: FocusMode::Flow,
            workspace_name: "Main Deck".to_string(),
            pinned_actions: vec![
                "Open terminal".to_string(),
                "Resume last project".to_string(),
                "Start focus timer".to_string(),
            ],
        }
    }

    pub fn set_focus(&mut self, mode: FocusMode) -> String {
        self.current_focus = mode.clone();
        let behavior = match mode {
            FocusMode::Flow => "balanced notifications and smooth multitasking",
            FocusMode::DeepWork => "notifications muted and CPU tuned for sustained concentration",
            FocusMode::Meeting => "camera/mic quick controls surfaced and distractions hidden",
            FocusMode::Recovery => "low-pressure layout with gentle reminders",
        };
        format!("Focus mode set to {:?}: {}.", mode, behavior)
    }

    pub fn soul_dashboard(&self) -> String {
        format!(
            "Workspace: {} | Focus: {:?} | Pinned: {}",
            self.workspace_name,
            self.current_focus,
            self.pinned_actions.join(" • ")
        )
    }

    pub fn command_palette(&self, query: &str) -> Vec<String> {
        let q = query.to_lowercase();
        self.pinned_actions
            .iter()
            .filter(|a| a.to_lowercase().contains(&q))
            .cloned()
            .collect()
    }
}
