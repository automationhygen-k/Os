use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct DiagnosticsCenter {
    log_path: PathBuf,
}

impl DiagnosticsCenter {
    pub fn new() -> Self {
        Self {
            log_path: PathBuf::from("/tmp/aetheros-diagnostics.log"),
        }
    }

    pub fn record(&self, level: &str, message: &str) {
        let _ = self.rotate_if_needed();
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)
        {
            let ts = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            let _ = writeln!(file, "[{ts}] [{level}] {message}");
        }
    }

    pub fn read_recent(&self, max_lines: usize) -> String {
        let Ok(text) = fs::read_to_string(&self.log_path) else {
            return "No diagnostics log yet.".to_string();
        };
        let lines = text.lines().collect::<Vec<_>>();
        let start = lines.len().saturating_sub(max_lines);
        lines[start..].join("\n")
    }

    pub fn clear(&self) -> String {
        match fs::remove_file(&self.log_path) {
            Ok(_) => "Diagnostics log cleared.".to_string(),
            Err(_) => "Diagnostics log already clear.".to_string(),
        }
    }

    fn rotate_if_needed(&self) -> std::io::Result<()> {
        const MAX: u64 = 512 * 1024;
        if let Ok(meta) = fs::metadata(&self.log_path) {
            if meta.len() > MAX {
                let backup = self.log_path.with_extension("log.1");
                let _ = fs::remove_file(&backup);
                fs::rename(&self.log_path, backup)?;
            }
        }
        Ok(())
    }
}
