#[derive(Clone)]
pub struct ContinuityCenter {
    pub last_clipboard: Option<String>,
    pub handoff_queue: Vec<String>,
}

impl ContinuityCenter {
    pub fn new() -> Self {
        Self {
            last_clipboard: None,
            handoff_queue: Vec::new(),
        }
    }

    pub fn airshare_clipboard(&mut self, payload: &str, device: &str) -> String {
        self.last_clipboard = Some(payload.to_string());
        format!(
            "AirShare delivered clipboard snippet to '{}' ({} chars).",
            device,
            payload.chars().count()
        )
    }

    pub fn handoff_task(&mut self, task: &str, target_device: &str) -> String {
        let record = format!("{} -> {}", task, target_device);
        self.handoff_queue.push(record.clone());
        format!("Handoff queued: {record}")
    }

    pub fn dynamic_orb(&self) -> String {
        let clip = self
            .last_clipboard
            .as_ref()
            .map(|v| format!("clipboard_ready({} chars)", v.len()))
            .unwrap_or_else(|| "clipboard_idle".to_string());
        format!(
            "Dynamic Orb: {} | pending handoffs={}.",
            clip,
            self.handoff_queue.len()
        )
    }
}
