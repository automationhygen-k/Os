#[derive(Clone)]
pub struct HumorPolicy {
    used_lines: Vec<String>,
}

impl HumorPolicy {
    pub fn new() -> Self {
        Self {
            used_lines: Vec::new(),
        }
    }

    pub fn maybe_line(
        &mut self,
        confidence: f32,
        system_constrained: bool,
        critical: bool,
        casual: bool,
        repeated_harmless: bool,
        after_friction_success: bool,
    ) -> Option<String> {
        if critical || system_constrained || confidence < 0.90 {
            return None;
        }
        if !(casual || repeated_harmless || after_friction_success) {
            return None;
        }

        let candidates = [
            "That was efficient. I checked twice, mostly for dramatic restraint.",
            "Completed. No fanfare, just competence.",
            "Done. The CPU remained calm about it.",
        ];

        for line in candidates {
            if !self.used_lines.iter().any(|u| u == line) {
                self.used_lines.push(line.to_string());
                return Some(line.to_string());
            }
        }
        None
    }
}
