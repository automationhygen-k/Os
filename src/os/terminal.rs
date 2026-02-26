use std::process::Command;

#[derive(Clone, Default)]
pub struct TerminalBridge;

impl TerminalBridge {
    pub fn run(&self, command: &str) -> Result<String, String> {
        let output = Command::new("bash")
            .arg("-lc")
            .arg(command)
            .output()
            .map_err(|e| format!("terminal failed: {e}"))?;

        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

        if output.status.success() {
            if stdout.is_empty() {
                Ok("Command completed successfully.".to_string())
            } else {
                Ok(stdout)
            }
        } else if stderr.is_empty() {
            Err(format!("Command failed with status {}", output.status))
        } else {
            Err(stderr)
        }
    }
}
