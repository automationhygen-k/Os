use std::collections::HashMap;

#[derive(Clone)]
pub struct ContextEngine {
    pub foreground_app: String,
    pub recent_actions: Vec<String>,
    friction_counts: HashMap<String, u32>,
    ignored_suggestions: HashMap<String, u8>,
}

impl ContextEngine {
    pub fn new() -> Self {
        Self {
            foreground_app: "Desktop".to_string(),
            recent_actions: Vec::new(),
            friction_counts: HashMap::new(),
            ignored_suggestions: HashMap::new(),
        }
    }

    pub fn set_foreground_app(&mut self, app: &str) {
        self.foreground_app = app.to_string();
    }

    pub fn record_action(&mut self, action: &str, friction: bool) {
        self.recent_actions.push(action.to_string());
        if self.recent_actions.len() > 20 {
            self.recent_actions.remove(0);
        }
        if friction {
            *self.friction_counts.entry(action.to_string()).or_insert(0) += 1;
        }
    }

    pub fn suggest(&self, candidate: &str, confidence: f32) -> Option<String> {
        if confidence < 0.80 {
            return None;
        }
        if self
            .ignored_suggestions
            .get(candidate)
            .copied()
            .unwrap_or(0)
            >= 2
        {
            return None;
        }
        Some(format!(
            "Suggestion: {candidate} (confidence {:.0}%)",
            confidence * 100.0
        ))
    }

    pub fn mark_ignored(&mut self, candidate: &str) {
        *self
            .ignored_suggestions
            .entry(candidate.to_string())
            .or_insert(0) += 1;
    }

    pub fn friction_summary(&self) -> String {
        if self.friction_counts.is_empty() {
            return "No friction hotspots detected.".to_string();
        }
        let mut rows = self
            .friction_counts
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect::<Vec<_>>();
        rows.sort_by(|a, b| b.1.cmp(&a.1));
        let top = rows
            .iter()
            .take(3)
            .map(|(k, v)| format!("{k}({v})"))
            .collect::<Vec<_>>()
            .join(" | ");
        format!("Friction hotspots: {top}")
    }
}
