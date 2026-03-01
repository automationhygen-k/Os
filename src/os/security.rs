#[derive(Clone)]
pub struct SecurityProfile {
    pub name: String,
    pub secure_boot: bool,
    pub memory_isolation: bool,
    pub signed_components_only: bool,
    pub sandbox_required: bool,
    pub network_egress_filtered: bool,
}

impl SecurityProfile {
    pub fn hardened_mobile_like() -> Self {
        Self {
            name: "Mobile-Grade Hardened".to_string(),
            secure_boot: true,
            memory_isolation: true,
            signed_components_only: true,
            sandbox_required: true,
            network_egress_filtered: true,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum Capability {
    ControlAudio,
    ControlDisplay,
    ControlNetworking,
    LaunchApps,
    InstallApps,
    ManageProcesses,
    UseTerminal,
    ManageFiles,
    ManageDesktop,
    ManageSettings,
    ManageNotifications,
}

#[derive(Clone)]
pub struct SandboxPolicy {
    pub can_access_home: bool,
    pub can_access_network: bool,
    pub allowed_env: Vec<String>,
    pub working_dir: Option<String>,
}

impl SandboxPolicy {
    pub fn strict_default() -> Self {
        Self {
            can_access_home: false,
            can_access_network: false,
            allowed_env: vec!["PATH".to_string(), "LANG".to_string()],
            working_dir: None,
        }
    }

    pub fn developer_friendly() -> Self {
        Self {
            can_access_home: true,
            can_access_network: true,
            allowed_env: vec!["PATH".to_string(), "LANG".to_string(), "HOME".to_string()],
            working_dir: None,
        }
    }
}
