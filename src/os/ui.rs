#[derive(Clone)]
pub struct ExperienceTheme {
    pub name: String,
    pub style: AestheticStyle,
    pub animations_enabled: bool,
    pub focus_assist: bool,
}

impl ExperienceTheme {
    pub fn new(name: &str, style: AestheticStyle) -> Self {
        Self {
            name: name.to_string(),
            style,
            animations_enabled: true,
            focus_assist: true,
        }
    }

    pub fn interaction_goal(&self) -> &'static str {
        match self.style {
            AestheticStyle::GlassDepth => "fluid, depth-rich and calm",
            AestheticStyle::Minimal => "clean and distraction-free",
            AestheticStyle::Dynamic => "adaptive and context-aware",
        }
    }

    pub fn describe(&self) -> String {
        format!(
            "Theme '{}' ({}) | animations: {} | focus assist: {}",
            self.name,
            self.interaction_goal(),
            self.animations_enabled,
            self.focus_assist
        )
    }
}

#[derive(Clone)]
pub enum AestheticStyle {
    GlassDepth,
    Minimal,
    Dynamic,
}
