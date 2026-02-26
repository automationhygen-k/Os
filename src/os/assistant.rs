use std::process::Command;

use crate::os::humor::HumorPolicy;
use crate::os::kernel::Kernel;
use crate::os::llm::{Intent, TinyLlm};
use crate::os::security::Capability;

pub struct Assistant {
    name: String,
    kernel: Kernel,
    llm: TinyLlm,
    humor: HumorPolicy,
}

impl Assistant {
    pub fn new(name: &str, kernel: Kernel) -> Self {
        Self {
            name: name.to_string(),
            kernel,
            llm: TinyLlm::embedded_default(),
            humor: HumorPolicy::new(),
        }
    }

    pub fn handle_command(&mut self, input: &str) -> Result<String, String> {
        if !self.kernel.is_booted() {
            return Err("System not ready. Next step: complete boot.".to_string());
        }

        let answer = match self.llm.infer_intent(input) {
            Intent::SetVolume(level) => self.control_volume(level),
            Intent::SetBrightness(level) => self.control_brightness(level),
            Intent::LaunchApp(name) => {
                let app_name = name
                    .strip_prefix("app ")
                    .unwrap_or(&name)
                    .trim()
                    .to_string();
                if app_name.is_empty() {
                    return Ok(format!("{}: launch requires an app name.", self.name));
                }
                let result = self.kernel.launch_app(&app_name)?;
                Ok(format!("{} launched '{}': {}", self.name, app_name, result))
            }
            Intent::ListApps => {
                let apps = self.kernel.list_apps()?;
                Ok(if apps.is_empty() {
                    format!("{}: no apps installed.", self.name)
                } else {
                    format!("{} apps:\n- {}", self.name, apps.join("\n- "))
                })
            }
            Intent::ListProcesses => {
                let processes = self.kernel.list_processes()?;
                Ok(if processes.is_empty() {
                    format!("{}: no process telemetry.", self.name)
                } else {
                    format!(
                        "{} processes:\n- {}",
                        self.name,
                        processes
                            .iter()
                            .map(|p| format!(
                                "pid={} cmd={} {}",
                                p.pid,
                                p.program,
                                p.args.join(" ")
                            ))
                            .collect::<Vec<_>>()
                            .join("\n- ")
                    )
                })
            }
            Intent::SetFocus(mode) => Ok(format!(
                "{}: {}",
                self.name,
                self.kernel.set_focus_mode(mode)
            )),
            Intent::SetPowerProfile(profile) => Ok(format!(
                "{}: {}",
                self.name,
                self.kernel.set_power_profile(profile)
            )),
            Intent::AirShare(payload) => Ok(format!(
                "{}: {}",
                self.name,
                self.kernel.airshare_clipboard(&payload, "Paired Device")?
            )),
            Intent::Handoff(task) => Ok(format!(
                "{}: {}",
                self.name,
                self.kernel.handoff(&task, "Desk Companion")?
            )),
            Intent::DynamicOrb => Ok(format!("{}: {}", self.name, self.kernel.dynamic_orb())),
            Intent::SoulDashboard => Ok(format!("{}: {}", self.name, self.kernel.soul_dashboard())),
            Intent::CommandPalette(query) => {
                let items = self.kernel.command_palette(&query);
                Ok(if items.is_empty() {
                    format!("{}: no palette match.", self.name)
                } else {
                    format!("{} palette:\n- {}", self.name, items.join("\n- "))
                })
            }
            Intent::Terminal(command) => Ok(format!(
                "{} terminal:\n{}",
                self.name,
                self.kernel.terminal_run(&command)?
            )),
            Intent::DockSummary => Ok(format!("{}: {}", self.name, self.kernel.dock_summary()?)),
            Intent::DockPin(app) => Ok(format!("{}: {}", self.name, self.kernel.dock_pin(&app)?)),
            Intent::OpenWindow(title) => Ok(format!(
                "{}: {}",
                self.name,
                self.kernel.open_window(&title)?
            )),
            Intent::CloseWindow(id) => {
                Ok(format!("{}: {}", self.name, self.kernel.close_window(id)?))
            }
            Intent::SwitchWorkspace(name) => Ok(format!(
                "{}: {}",
                self.name,
                self.kernel.switch_workspace(&name)?
            )),
            Intent::MissionControl => {
                Ok(format!("{}: {}", self.name, self.kernel.mission_control()?))
            }
            Intent::InteractionStatus(ms) => Ok(format!(
                "{}: {}",
                self.name,
                self.kernel.interaction_status(ms)
            )),
            Intent::DesignIntegrity => Ok(format!(
                "{}: {}",
                self.name,
                self.kernel.explain_integrity()
            )),
            Intent::Suggest(candidate) => Ok(format!(
                "{}: {}",
                self.name,
                self.kernel.suggestion(&candidate, 0.86)
            )),
            Intent::IgnoreSuggestion(candidate) => {
                self.kernel.ignore_suggestion(&candidate);
                Ok(format!("{}: suggestion '{}' muted.", self.name, candidate))
            }
            Intent::FilesList(path) => Ok(format!(
                "{} files: {}",
                self.name,
                self.kernel.file_list(path.as_deref())?
            )),
            Intent::FilesRead(path) => Ok(format!(
                "{} read:\n{}",
                self.name,
                self.kernel.file_read(&path)?
            )),
            Intent::FilesWrite(path, content) => Ok(format!(
                "{}: {}",
                self.name,
                self.kernel.file_write(&path, &content)?
            )),
            Intent::FilesSearch(query) => Ok(format!(
                "{}: {}",
                self.name,
                self.kernel.file_search(&query, 8)?
            )),
            Intent::SettingsSummary => Ok(format!(
                "{} settings: {}",
                self.name,
                self.kernel.settings_summary()?
            )),
            Intent::SetAccent(color) => Ok(format!(
                "{}: {}",
                self.name,
                self.kernel.set_accent_color(&color)?
            )),
            Intent::ToggleNightShift => Ok(format!(
                "{}: {}",
                self.name,
                self.kernel.toggle_night_shift()?
            )),
            Intent::BatterySaver(enabled) => Ok(format!(
                "{}: {}",
                self.name,
                self.kernel.set_battery_saver(enabled)?
            )),
            Intent::Notify(message) => {
                Ok(format!("{}: {}", self.name, self.kernel.notify(&message)?))
            }
            Intent::NotifyLow(message) => Ok(format!(
                "{}: {}",
                self.name,
                self.kernel.notify_low_priority(&message)?
            )),
            Intent::FlushDigest => Ok(format!(
                "{}: {}",
                self.name,
                self.kernel.flush_notification_digest()?
            )),
            Intent::Notifications => Ok(format!(
                "{}: {}",
                self.name,
                self.kernel.notifications_inbox()?
            )),
            Intent::Dnd(enabled) => Ok(format!("{}: {}", self.name, self.kernel.set_dnd(enabled)?)),
            Intent::Casual(question) => Ok(self.handle_casual(&question)),
            Intent::AskWeb(question) => {
                let web_answer = self.query_web(&question).unwrap_or_else(|_| {
                    "Web lookup unavailable. Next step: retry when network stabilizes.".to_string()
                });
                Ok(format!(
                    "{} [{} ctx={}]: {}",
                    self.name, self.llm.name, self.llm.max_context_tokens, web_answer
                ))
            }
        }?;

        let maybe_wit = self.humor.maybe_line(
            0.95,
            false,
            false,
            input.to_lowercase().contains("who are you") || input.to_lowercase().contains("joke"),
            false,
            input.to_lowercase().contains("flush digest"),
        );

        if let Some(line) = maybe_wit {
            Ok(format!("{}\n{}", answer, line))
        } else {
            Ok(answer)
        }
    }

    fn handle_casual(&self, q: &str) -> String {
        if q.to_lowercase().contains("who are you") {
            return format!(
                "{}: I optimize tasks, explain behavior, and avoid guessing capabilities.",
                self.name
            );
        }
        format!(
            "{}: I can help. Ask directly and I will keep it concise.",
            self.name
        )
    }

    pub fn install_app(
        &mut self,
        name: &str,
        entry: &str,
        args: Vec<String>,
    ) -> Result<String, String> {
        self.kernel.install_app(name, entry, args)
    }

    pub fn seed_demo_apps(&mut self) {
        self.kernel.seed_demo_apps();
    }

    fn control_volume(&self, level: u8) -> Result<String, String> {
        if !self.kernel.has_capability(&Capability::ControlAudio) {
            return Err("Volume change unavailable. Next step: grant audio control.".to_string());
        }
        Ok(format!("{} set system volume to {}%.", self.name, level))
    }

    fn control_brightness(&self, level: u8) -> Result<String, String> {
        if !self.kernel.has_capability(&Capability::ControlDisplay) {
            return Err(
                "Brightness change unavailable. Next step: grant display control.".to_string(),
            );
        }
        Ok(format!(
            "{} set display brightness to {}%.",
            self.name, level
        ))
    }

    fn query_web(&self, question: &str) -> Result<String, String> {
        if !self.kernel.has_capability(&Capability::ControlNetworking) {
            return Err("Web lookup unavailable. Next step: grant network control.".to_string());
        }

        let output = Command::new("curl")
            .arg("-sL")
            .arg("https://duckduckgo.com/html/")
            .arg("--data-urlencode")
            .arg(format!("q={question}"))
            .output()
            .map_err(|e| e.to_string())?;

        if !output.status.success() {
            return Err("Web query failed. Next step: retry.".to_string());
        }

        let text = String::from_utf8_lossy(&output.stdout);
        if text.is_empty() {
            return Ok("No web response text returned.".to_string());
        }

        Ok(extract_title_like_line(&text)
            .unwrap_or_else(|| "Fetched web results. Parsing can be expanded.".to_string()))
    }
}

fn extract_title_like_line(html: &str) -> Option<String> {
    html.lines()
        .find(|line| line.contains("result__a") || line.contains("<title>"))
        .map(|line| {
            line.replace("<title>", "")
                .replace("</title>", "")
                .replace("<a", " ")
                .replace("</a>", "")
                .replace('"', "")
                .trim()
                .to_string()
        })
        .filter(|line| !line.is_empty())
}
