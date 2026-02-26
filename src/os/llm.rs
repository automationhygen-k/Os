use crate::os::performance::PowerProfile;
use crate::os::productivity::FocusMode;

#[derive(Clone)]
pub struct TinyLlm {
    pub name: String,
    pub max_context_tokens: usize,
}

impl TinyLlm {
    pub fn embedded_default() -> Self {
        Self {
            name: "Nova-Tiny".to_string(),
            max_context_tokens: 512,
        }
    }

    pub fn infer_intent(&self, input: &str) -> Intent {
        let lower = input.to_lowercase();

        if lower.contains("set volume") || lower.contains("volume") {
            return Intent::SetVolume(extract_percent(input).unwrap_or(50));
        }
        if lower.contains("brightness") {
            return Intent::SetBrightness(extract_percent(input).unwrap_or(60));
        }
        if lower.starts_with("run ") || lower.starts_with("launch ") {
            let payload = input
                .split_once(' ')
                .map(|(_, rest)| rest.trim().to_string())
                .unwrap_or_default();
            return Intent::LaunchApp(payload);
        }
        if lower.contains("list apps") || lower.contains("installed apps") {
            return Intent::ListApps;
        }
        if lower.contains("list processes") {
            return Intent::ListProcesses;
        }

        if lower.contains("focus deep") {
            return Intent::SetFocus(FocusMode::DeepWork);
        }
        if lower.contains("focus meeting") {
            return Intent::SetFocus(FocusMode::Meeting);
        }
        if lower.contains("focus recovery") {
            return Intent::SetFocus(FocusMode::Recovery);
        }
        if lower.contains("focus flow") || lower.contains("set focus") {
            return Intent::SetFocus(FocusMode::Flow);
        }

        if lower.contains("power silent") {
            return Intent::SetPowerProfile(PowerProfile::Silent);
        }
        if lower.contains("power balanced") {
            return Intent::SetPowerProfile(PowerProfile::Balanced);
        }
        if lower.contains("power creator") {
            return Intent::SetPowerProfile(PowerProfile::Creator);
        }
        if lower.contains("power beast") || lower.contains("game mode") {
            return Intent::SetPowerProfile(PowerProfile::Beast);
        }

        if let Some(rest) = input.strip_prefix("terminal ") {
            return Intent::Terminal(rest.trim().to_string());
        }
        if lower == "dock" {
            return Intent::DockSummary;
        }
        if let Some(rest) = input.strip_prefix("dock pin ") {
            return Intent::DockPin(rest.trim().to_string());
        }
        if let Some(rest) = input.strip_prefix("open window ") {
            return Intent::OpenWindow(rest.trim().to_string());
        }
        if let Some(rest) = lower.strip_prefix("close window ") {
            if let Ok(id) = rest.trim().parse::<u32>() {
                return Intent::CloseWindow(id);
            }
        }
        if let Some(rest) = input.strip_prefix("workspace ") {
            return Intent::SwitchWorkspace(rest.trim().to_string());
        }
        if lower == "mission control" {
            return Intent::MissionControl;
        }

        if let Some(rest) = lower.strip_prefix("airshare ") {
            return Intent::AirShare(rest.trim().to_string());
        }
        if let Some(rest) = lower.strip_prefix("handoff ") {
            return Intent::Handoff(rest.trim().to_string());
        }
        if lower.contains("orb") || lower.contains("dynamic island") {
            return Intent::DynamicOrb;
        }
        if lower == "dashboard" || lower == "soul dashboard" {
            return Intent::SoulDashboard;
        }
        if let Some(rest) = lower.strip_prefix("palette ") {
            return Intent::CommandPalette(rest.trim().to_string());
        }

        if lower.starts_with("latency ") {
            if let Some(ms) = lower
                .split_whitespace()
                .nth(1)
                .and_then(|v| v.parse::<u64>().ok())
            {
                return Intent::InteractionStatus(ms);
            }
        }
        if lower == "design integrity" {
            return Intent::DesignIntegrity;
        }
        if lower.starts_with("suggest ") {
            let rest = input
                .strip_prefix("suggest ")
                .unwrap_or("")
                .trim()
                .to_string();
            return Intent::Suggest(rest);
        }
        if lower.starts_with("ignore suggestion ") {
            return Intent::IgnoreSuggestion(
                input
                    .strip_prefix("ignore suggestion ")
                    .unwrap_or("")
                    .trim()
                    .to_string(),
            );
        }

        if lower == "files list" {
            return Intent::FilesList(None);
        }
        if let Some(rest) = input.strip_prefix("files list ") {
            return Intent::FilesList(Some(rest.trim().to_string()));
        }
        if let Some(rest) = input.strip_prefix("files read ") {
            return Intent::FilesRead(rest.trim().to_string());
        }
        if let Some(rest) = input.strip_prefix("files write ") {
            if let Some((path, content)) = rest.split_once("::") {
                return Intent::FilesWrite(path.trim().to_string(), content.trim().to_string());
            }
        }
        if let Some(rest) = input.strip_prefix("files search ") {
            return Intent::FilesSearch(rest.trim().to_string());
        }

        if lower == "settings" {
            return Intent::SettingsSummary;
        }
        if let Some(rest) = input.strip_prefix("set accent ") {
            return Intent::SetAccent(rest.trim().to_string());
        }
        if lower == "toggle night shift" {
            return Intent::ToggleNightShift;
        }
        if lower == "battery saver on" {
            return Intent::BatterySaver(true);
        }
        if lower == "battery saver off" {
            return Intent::BatterySaver(false);
        }

        if let Some(rest) = input.strip_prefix("notify low ") {
            return Intent::NotifyLow(rest.trim().to_string());
        }
        if let Some(rest) = input.strip_prefix("notify ") {
            return Intent::Notify(rest.trim().to_string());
        }
        if lower == "flush digest" {
            return Intent::FlushDigest;
        }
        if lower == "notifications" {
            return Intent::Notifications;
        }
        if lower == "dnd on" {
            return Intent::Dnd(true);
        }
        if lower == "dnd off" {
            return Intent::Dnd(false);
        }

        if lower.starts_with("can you")
            || lower.starts_with("who are you")
            || lower.contains("joke")
        {
            return Intent::Casual(input.to_string());
        }

        Intent::AskWeb(input.to_string())
    }
}

#[derive(Clone)]
pub enum Intent {
    SetVolume(u8),
    SetBrightness(u8),
    LaunchApp(String),
    ListApps,
    ListProcesses,
    SetFocus(FocusMode),
    SetPowerProfile(PowerProfile),
    AirShare(String),
    Handoff(String),
    DynamicOrb,
    SoulDashboard,
    CommandPalette(String),
    Terminal(String),
    DockSummary,
    DockPin(String),
    OpenWindow(String),
    CloseWindow(u32),
    SwitchWorkspace(String),
    MissionControl,
    InteractionStatus(u64),
    DesignIntegrity,
    Suggest(String),
    IgnoreSuggestion(String),
    FilesList(Option<String>),
    FilesRead(String),
    FilesWrite(String, String),
    FilesSearch(String),
    SettingsSummary,
    SetAccent(String),
    ToggleNightShift,
    BatterySaver(bool),
    Notify(String),
    NotifyLow(String),
    FlushDigest,
    Notifications,
    Dnd(bool),
    Casual(String),
    AskWeb(String),
}

fn extract_percent(input: &str) -> Option<u8> {
    input
        .split_whitespace()
        .find_map(|token| token.parse::<u8>().ok())
        .map(|value| value.min(100))
}

#[cfg(test)]
mod tests {
    use super::{Intent, TinyLlm};

    #[test]
    fn parses_terminal_and_file_commands() {
        let llm = TinyLlm::embedded_default();

        match llm.infer_intent("terminal echo hi") {
            Intent::Terminal(cmd) => assert_eq!(cmd, "echo hi"),
            _ => panic!("expected terminal intent"),
        }

        match llm.infer_intent("files write demo.txt :: body") {
            Intent::FilesWrite(path, body) => {
                assert_eq!(path, "demo.txt");
                assert_eq!(body, "body");
            }
            _ => panic!("expected files write intent"),
        }
    }
}
