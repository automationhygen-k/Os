use std::collections::HashMap;
use std::process::{Command, Stdio};

#[derive(Clone)]
pub struct ProcessRecord {
    pub pid: u32,
    pub program: String,
    pub args: Vec<String>,
}

#[derive(Clone, Default)]
pub struct ProcessManager {
    processes: HashMap<u32, ProcessRecord>,
    next_pid: u32,
}

impl ProcessManager {
    pub fn spawn_and_wait(&mut self, program: &str, args: &[String]) -> Result<String, String> {
        let output = Command::new(program)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| format!("failed to spawn '{}': {e}", program))?;

        let pid = self.alloc_pid();
        self.processes.insert(
            pid,
            ProcessRecord {
                pid,
                program: program.to_string(),
                args: args.to_vec(),
            },
        );

        if output.status.success() {
            let out = String::from_utf8_lossy(&output.stdout).trim().to_string();
            Ok(if out.is_empty() {
                format!("Process '{}' (pid={}) completed.", program, pid)
            } else {
                out
            })
        } else {
            let err = String::from_utf8_lossy(&output.stderr).trim().to_string();
            Err(if err.is_empty() {
                format!(
                    "Process '{}' (pid={}) failed with status {}",
                    program, pid, output.status
                )
            } else {
                err
            })
        }
    }

    pub fn list_processes(&self) -> Vec<ProcessRecord> {
        let mut rows = self.processes.values().cloned().collect::<Vec<_>>();
        rows.sort_by_key(|p| p.pid);
        rows
    }

    fn alloc_pid(&mut self) -> u32 {
        self.next_pid = self.next_pid.saturating_add(1).max(1);
        self.next_pid
    }
}
