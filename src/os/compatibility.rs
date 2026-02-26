use std::path::Path;
use std::process::Command;

use crate::os::app::{AppManifest, Platform};

#[derive(Clone, Default)]
pub struct CompatibilityLayer;

impl CompatibilityLayer {
    pub fn launch_command(&self, app: &AppManifest) -> Result<(String, Vec<String>), String> {
        match app.platform {
            Platform::Native => Ok((app.entry.clone(), app.args.clone())),
            Platform::Windows => {
                if command_exists("wine") {
                    let mut args = vec![app.entry.clone()];
                    args.extend(app.args.clone());
                    Ok(("wine".to_string(), args))
                } else {
                    Err("Windows app requested but 'wine' is not installed.".to_string())
                }
            }
            Platform::MacOS => {
                if command_exists("darling") {
                    let mut args = vec!["shell".to_string(), app.entry.clone()];
                    args.extend(app.args.clone());
                    Ok(("darling".to_string(), args))
                } else {
                    Err("macOS app requested but 'darling' is not installed.".to_string())
                }
            }
            Platform::Android => {
                if command_exists("waydroid") {
                    let mut args = vec!["app".to_string(), "launch".to_string(), app.entry.clone()];
                    args.extend(app.args.clone());
                    Ok(("waydroid".to_string(), args))
                } else if command_exists("adb") {
                    let mut args = vec![
                        "shell".to_string(),
                        "am".to_string(),
                        "start".to_string(),
                        "-n".to_string(),
                        app.entry.clone(),
                    ];
                    args.extend(app.args.clone());
                    Ok(("adb".to_string(), args))
                } else {
                    Err(
                        "Android app requested but neither 'waydroid' nor 'adb' is installed."
                            .to_string(),
                    )
                }
            }
        }
    }
}

fn command_exists(command: &str) -> bool {
    Command::new("bash")
        .arg("-lc")
        .arg(format!("command -v {command} >/dev/null 2>&1"))
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn platform_from_path(path: &str) -> Platform {
    if path.ends_with(".exe") {
        Platform::Windows
    } else if path.ends_with(".apk") {
        Platform::Android
    } else if Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| e == "app")
    {
        Platform::MacOS
    } else {
        Platform::Native
    }
}
