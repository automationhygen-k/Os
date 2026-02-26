#[derive(Clone)]
pub struct NotificationCenter {
    do_not_disturb: bool,
    queue: Vec<String>,
    digest_queue: Vec<String>,
}

impl NotificationCenter {
    pub fn new() -> Self {
        Self {
            do_not_disturb: false,
            queue: Vec::new(),
            digest_queue: Vec::new(),
        }
    }

    pub fn set_dnd(&mut self, enabled: bool) -> String {
        self.do_not_disturb = enabled;
        format!("Do Not Disturb: {}", self.do_not_disturb)
    }

    pub fn push(&mut self, message: &str) -> String {
        self.queue.push(message.to_string());
        self.expire_non_critical();
        if self.do_not_disturb {
            "Notification queued silently (DND enabled).".to_string()
        } else {
            format!("Notification delivered: {message}")
        }
    }

    pub fn push_low_priority(&mut self, message: &str) {
        self.digest_queue.push(message.to_string());
    }

    pub fn flush_digest(&mut self) -> String {
        if self.digest_queue.is_empty() {
            return "No low-priority digest items.".to_string();
        }
        let joined = self.digest_queue.join(" | ");
        self.digest_queue.clear();
        self.queue.push(format!("Digest: {joined}"));
        self.expire_non_critical();
        "Low-priority digest published.".to_string()
    }

    pub fn inbox(&self) -> String {
        if self.queue.is_empty() {
            return "No notifications.".to_string();
        }
        format!("Notifications: {}", self.queue.join(" | "))
    }

    fn expire_non_critical(&mut self) {
        let keep = 12;
        if self.queue.len() > keep {
            let drop_count = self.queue.len() - keep;
            self.queue.drain(0..drop_count);
        }
    }
}
