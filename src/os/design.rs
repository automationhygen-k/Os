use std::time::Duration;

#[derive(Clone)]
pub struct InteractionRulebook {
    pub input_ack_ms: (u64, u64),
    pub perceived_response_ms: u64,
    pub progressive_feedback_ms: u64,
    pub cancellable_ms: u64,
    pub animation_ms: (u64, u64),
    pub grid_unit: u8,
    pub max_font_weights: u8,
}

impl InteractionRulebook {
    pub fn next_gen_default() -> Self {
        Self {
            input_ack_ms: (8, 16),
            perceived_response_ms: 50,
            progressive_feedback_ms: 120,
            cancellable_ms: 400,
            animation_ms: (120, 240),
            grid_unit: 8,
            max_font_weights: 3,
        }
    }

    pub fn operation_policy(&self, elapsed: Duration) -> OperationPolicy {
        let elapsed_ms = elapsed.as_millis() as u64;
        OperationPolicy {
            should_ack_immediately: elapsed_ms <= self.input_ack_ms.1,
            should_show_progressive_feedback: elapsed_ms >= self.progressive_feedback_ms,
            should_allow_cancel: elapsed_ms >= self.cancellable_ms,
            violating_perceptual_budget: elapsed_ms > self.perceived_response_ms,
        }
    }

    pub fn motion_quality_for_load(&self, cpu_load: f32) -> MotionComplexity {
        if cpu_load > 0.85 {
            MotionComplexity::Reduced
        } else if cpu_load > 0.65 {
            MotionComplexity::Moderate
        } else {
            MotionComplexity::Full
        }
    }

    pub fn concise_error(&self, message: &str) -> String {
        let mut words = message.split_whitespace().take(12).collect::<Vec<_>>();
        if words.is_empty() {
            return "Operation failed. Retry once.".to_string();
        }
        if words.len() == 12 {
            words[11] = "…";
        }
        words.join(" ")
    }
}

#[derive(Clone)]
pub struct OperationPolicy {
    pub should_ack_immediately: bool,
    pub should_show_progressive_feedback: bool,
    pub should_allow_cancel: bool,
    pub violating_perceptual_budget: bool,
}

#[derive(Clone)]
pub enum MotionComplexity {
    Full,
    Moderate,
    Reduced,
}
