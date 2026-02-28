use std::time::{Duration, Instant};

use crate::os::app::{AppManifest, AppRegistry, Platform};
use crate::os::compatibility::{platform_from_path, CompatibilityLayer};
use crate::os::context::ContextEngine;
use crate::os::continuity::ContinuityCenter;
use crate::os::design::{InteractionRulebook, MotionComplexity};
use crate::os::desktop::DesktopShell;
use crate::os::diagnostics::DiagnosticsCenter;
use crate::os::filesystem::FileExplorer;
use crate::os::notifications::NotificationCenter;
use crate::os::performance::{PerformanceDirector, PowerProfile};
use crate::os::process::{ProcessManager, ProcessRecord};
use crate::os::productivity::{FocusMode, Workbench};
use crate::os::security::{Capability, SandboxPolicy, SecurityProfile};
use crate::os::settings::SystemSettings;
use crate::os::terminal::TerminalBridge;
use crate::os::ui::ExperienceTheme;

#[derive(Clone)]
pub struct Kernel {
    pub name: String,
    profile: SecurityProfile,
    theme: ExperienceTheme,
    capabilities: Vec<Capability>,
    booted: bool,
    interaction_ready: bool,
    registry: AppRegistry,
    process_manager: ProcessManager,
    compatibility: CompatibilityLayer,
    performance: PerformanceDirector,
    productivity: Workbench,
    continuity: ContinuityCenter,
    desktop: DesktopShell,
    files: FileExplorer,
    terminal: TerminalBridge,
    settings: SystemSettings,
    notifications: NotificationCenter,
    context: ContextEngine,
    diagnostics: DiagnosticsCenter,
    rulebook: InteractionRulebook,
}

impl Kernel {
    pub fn new(name: &str, profile: SecurityProfile, theme: ExperienceTheme) -> Self {
        Self {
            name: name.to_string(),
            profile,
            theme,
            capabilities: Vec::new(),
            booted: false,
            interaction_ready: false,
            registry: AppRegistry::default(),
            process_manager: ProcessManager::default(),
            compatibility: CompatibilityLayer,
            performance: PerformanceDirector::new(),
            productivity: Workbench::new(),
            continuity: ContinuityCenter::new(),
            desktop: DesktopShell::new(),
            files: FileExplorer::new(),
            terminal: TerminalBridge,
            settings: SystemSettings::new(),
            notifications: NotificationCenter::new(),
            context: ContextEngine::new(),
            diagnostics: DiagnosticsCenter::new(),
            rulebook: InteractionRulebook::next_gen_default(),
        }
    }

    pub fn boot(&mut self) -> Result<(), String> {
        if !self.profile.secure_boot || !self.profile.memory_isolation {
            return Err("Kernel refused boot. Next step: verify secure profile.".to_string());
        }

        self.interaction_ready = true;
        self.booted = true;
        self.diagnostics
            .record("INFO", "kernel boot completed and interaction is ready");
        let tuning = self.performance.apply_profile(PowerProfile::Balanced);
        println!(
            "{} ready | {} | {} | workers={} latency={}ms | secure='{}'",
            self.name,
            self.theme.describe(),
            self.performance.describe(),
            tuning.worker_threads,
            tuning.frame_latency_target_ms,
            self.profile.name,
        );
        Ok(())
    }

    pub fn register_capability(&mut self, capability: Capability) {
        if !self.capabilities.contains(&capability) {
            self.capabilities.push(capability);
        }
    }

    pub fn has_capability(&self, capability: &Capability) -> bool {
        self.capabilities.contains(capability)
    }

    pub fn is_booted(&self) -> bool {
        self.booted
    }

    pub fn interaction_status(&self, elapsed_ms: u64) -> String {
        let policy = self
            .rulebook
            .operation_policy(Duration::from_millis(elapsed_ms));
        format!(
            "ack={} progressive={} cancellable={} budget_violation={}",
            policy.should_ack_immediately,
            policy.should_show_progressive_feedback,
            policy.should_allow_cancel,
            policy.violating_perceptual_budget
        )
    }

    pub fn motion_mode(&self) -> String {
        let cpu_hint = if self.performance.thread_budget <= 2 {
            0.85
        } else if self.performance.thread_budget <= 4 {
            0.65
        } else {
            0.40
        };
        let mode = match self.rulebook.motion_quality_for_load(cpu_hint) {
            MotionComplexity::Full => "full",
            MotionComplexity::Moderate => "moderate",
            MotionComplexity::Reduced => "reduced",
        };
        format!("motion={mode} | curve=global | duration=120..240ms")
    }

    pub fn explain_integrity(&self) -> String {
        format!(
            "grid={}pt | max_font_weights={} | perceptual_response={}ms",
            self.rulebook.grid_unit,
            self.rulebook.max_font_weights,
            self.rulebook.perceived_response_ms
        )
    }

    pub fn install_app(
        &mut self,
        name: &str,
        entry: &str,
        args: Vec<String>,
    ) -> Result<String, String> {
        self.require(&Capability::InstallApps)?;
        let platform = platform_from_path(entry);
        let app = AppManifest {
            id: format!("app.{}", name.to_lowercase().replace(' ', "_")),
            name: name.to_string(),
            platform,
            entry: entry.to_string(),
            args,
            sandbox: SandboxPolicy::strict_default(),
        };
        self.registry.register(app);
        self.context.record_action("install_app", false);
        Ok(format!("Installed app '{}'.", name))
    }

    pub fn list_apps(&self) -> Result<Vec<String>, String> {
        self.require(&Capability::LaunchApps)?;
        Ok(self
            .registry
            .list()
            .iter()
            .map(|a| {
                format!(
                    "{} ({:?}) -> {} | sandbox(home={}, net={}, env={})",
                    a.name,
                    a.platform,
                    a.entry,
                    a.sandbox.can_access_home,
                    a.sandbox.can_access_network,
                    a.sandbox.allowed_env.join(":"),
                )
            })
            .collect())
    }

    pub fn launch_app(&mut self, name: &str) -> Result<String, String> {
        self.require(&Capability::LaunchApps)?;
        let app = self
            .registry
            .find_by_name(name)
            .ok_or_else(|| "App unavailable. Next step: install and retry.".to_string())?;

        self.context.set_foreground_app(&app.name);
        let start = Instant::now();
        let (program, args) = self.compatibility.launch_command(&app)?;
        let mut policy = app.sandbox.clone();
        if policy.can_access_home {
            policy.working_dir = Some(self.files.home.to_string_lossy().to_string());
        }
        let result = self
            .process_manager
            .spawn_and_wait_sandboxed(&program, &args, &policy);
        if let Err(e) = &result {
            self.diagnostics
                .record("ERROR", &format!("launch_app failed: {e}"));
        }
        let elapsed = start.elapsed();
        self.context.record_action(
            "launch_app",
            elapsed.as_millis() as u64 > self.rulebook.perceived_response_ms,
        );
        result
    }

    pub fn list_processes(&self) -> Result<Vec<ProcessRecord>, String> {
        self.require(&Capability::ManageProcesses)?;
        Ok(self.process_manager.list_processes())
    }

    pub fn set_focus_mode(&mut self, mode: FocusMode) -> String {
        self.context.record_action("focus_mode", false);
        self.productivity.set_focus(mode)
    }

    pub fn soul_dashboard(&self) -> String {
        format!(
            "{} | {} | {}",
            self.productivity.soul_dashboard(),
            self.context.friction_summary(),
            self.motion_mode()
        )
    }

    pub fn command_palette(&self, query: &str) -> Vec<String> {
        self.productivity.command_palette(query)
    }

    pub fn set_power_profile(&mut self, profile: PowerProfile) -> String {
        let tuning = self.performance.apply_profile(profile.clone());
        format!(
            "Power profile {:?}: workers={}, io_boost={}, target={}ms",
            profile,
            tuning.worker_threads,
            tuning.io_priority_boost,
            tuning.frame_latency_target_ms
        )
    }

    pub fn airshare_clipboard(&mut self, payload: &str, device: &str) -> Result<String, String> {
        self.require(&Capability::ControlNetworking)?;
        Ok(self.continuity.airshare_clipboard(payload, device))
    }

    pub fn handoff(&mut self, task: &str, device: &str) -> Result<String, String> {
        self.require(&Capability::ControlNetworking)?;
        Ok(self.continuity.handoff_task(task, device))
    }

    pub fn dynamic_orb(&self) -> String {
        self.continuity.dynamic_orb()
    }

    pub fn terminal_run(&self, command: &str) -> Result<String, String> {
        self.require(&Capability::UseTerminal)?;
        self.terminal.run(command)
    }

    pub fn file_list(&self, path: Option<&str>) -> Result<String, String> {
        self.require(&Capability::ManageFiles)?;
        let entries = self.files.list(path)?;
        if entries.is_empty() {
            Ok("Folder is empty.".to_string())
        } else {
            Ok(entries
                .iter()
                .map(|e| format!("{}{}", e.name, if e.is_dir { "/" } else { "" }))
                .collect::<Vec<_>>()
                .join(" | "))
        }
    }

    pub fn file_read(&self, path: &str) -> Result<String, String> {
        self.require(&Capability::ManageFiles)?;
        self.files.read_text(path)
    }

    pub fn file_write(&self, path: &str, content: &str) -> Result<String, String> {
        self.require(&Capability::ManageFiles)?;
        self.files.write_text(path, content)
    }

    pub fn file_search(&self, query: &str, limit: usize) -> Result<String, String> {
        self.require(&Capability::ManageFiles)?;
        let hits = self.files.search_name(query, limit);
        if hits.is_empty() {
            Ok(format!("No files matched '{}'.", query))
        } else {
            Ok(format!("Found: {}", hits.join(" | ")))
        }
    }

    pub fn dock_summary(&self) -> Result<String, String> {
        self.require(&Capability::ManageDesktop)?;
        Ok(self.desktop.dock_summary())
    }

    pub fn dock_pin(&mut self, app: &str) -> Result<String, String> {
        self.require(&Capability::ManageDesktop)?;
        Ok(self.desktop.pin_to_dock(app))
    }

    pub fn open_all_apps_page(&self) -> Result<String, String> {
        self.require(&Capability::ManageDesktop)?;
        let names = self
            .registry
            .list()
            .iter()
            .map(|a| a.name.clone())
            .collect::<Vec<_>>();
        Ok(self.desktop.all_apps_page(&names))
    }

    pub fn open_window(&mut self, title: &str) -> Result<String, String> {
        self.require(&Capability::ManageDesktop)?;
        Ok(self.desktop.open_window(title))
    }

    pub fn close_window(&mut self, id: u32) -> Result<String, String> {
        self.require(&Capability::ManageDesktop)?;
        Ok(self.desktop.close_window(id))
    }

    pub fn switch_workspace(&mut self, name: &str) -> Result<String, String> {
        self.require(&Capability::ManageDesktop)?;
        Ok(self.desktop.switch_workspace(name))
    }

    pub fn mission_control(&self) -> Result<String, String> {
        self.require(&Capability::ManageDesktop)?;
        Ok(self.desktop.expose())
    }

    pub fn settings_summary(&self) -> Result<String, String> {
        self.require(&Capability::ManageSettings)?;
        Ok(self.settings.summary())
    }

    pub fn set_accent_color(&mut self, color: &str) -> Result<String, String> {
        self.require(&Capability::ManageSettings)?;
        Ok(self.settings.set_accent(color))
    }

    pub fn toggle_night_shift(&mut self) -> Result<String, String> {
        self.require(&Capability::ManageSettings)?;
        Ok(self.settings.toggle_night_shift())
    }

    pub fn set_battery_saver(&mut self, enabled: bool) -> Result<String, String> {
        self.require(&Capability::ManageSettings)?;
        Ok(self.settings.set_battery_saver(enabled))
    }

    pub fn set_dnd(&mut self, enabled: bool) -> Result<String, String> {
        self.require(&Capability::ManageNotifications)?;
        Ok(self.notifications.set_dnd(enabled))
    }

    pub fn notify(&mut self, message: &str) -> Result<String, String> {
        self.require(&Capability::ManageNotifications)?;
        Ok(self.notifications.push(message))
    }

    pub fn notify_low_priority(&mut self, message: &str) -> Result<String, String> {
        self.require(&Capability::ManageNotifications)?;
        self.notifications.push_low_priority(message);
        Ok("Low-priority alert queued for digest.".to_string())
    }

    pub fn flush_notification_digest(&mut self) -> Result<String, String> {
        self.require(&Capability::ManageNotifications)?;
        Ok(self.notifications.flush_digest())
    }

    pub fn notifications_inbox(&self) -> Result<String, String> {
        self.require(&Capability::ManageNotifications)?;
        Ok(self.notifications.inbox())
    }

    pub fn suggestion(&self, candidate: &str, confidence: f32) -> String {
        self.context
            .suggest(candidate, confidence)
            .unwrap_or_else(|| "No suggestion shown (confidence/attention policy).".to_string())
    }

    pub fn ignore_suggestion(&mut self, candidate: &str) {
        self.context.mark_ignored(candidate);
    }

    pub fn concise_error(&self, message: &str) -> String {
        self.rulebook.concise_error(message)
    }

    pub fn diagnostics_report(&self) -> String {
        self.diagnostics.read_recent(50)
    }

    pub fn diagnostics_clear(&self) -> String {
        self.diagnostics.clear()
    }

    pub fn diagnostics_record(&self, level: &str, message: &str) {
        self.diagnostics.record(level, message);
    }

    pub fn seed_demo_apps(&mut self) {
        self.registry.register(AppManifest {
            id: "app.echo_demo".to_string(),
            name: "Echo Demo".to_string(),
            platform: Platform::Native,
            entry: "echo".to_string(),
            args: vec!["Hello from AetherOS native app runtime".to_string()],
            sandbox: SandboxPolicy::strict_default(),
        });
    }

    fn require(&self, capability: &Capability) -> Result<(), String> {
        if self.has_capability(capability) {
            Ok(())
        } else {
            Err("Capability denied. Next step: grant permission.".to_string())
        }
    }
}
