use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct FileEntry {
    pub name: String,
    pub is_dir: bool,
}

#[derive(Clone)]
pub struct FileExplorer {
    pub home: PathBuf,
}

impl FileExplorer {
    pub fn new() -> Self {
        let base = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let home = base.join(".aether_home");
        let _ = fs::create_dir_all(&home);
        Self { home }
    }

    pub fn list(&self, path: Option<&str>) -> Result<Vec<FileEntry>, String> {
        let target = self.resolve(path.unwrap_or("."))?;
        let mut entries = fs::read_dir(&target)
            .map_err(|e| format!("failed to read '{}': {e}", target.display()))?
            .filter_map(Result::ok)
            .map(|de| {
                let name = de.file_name().to_string_lossy().to_string();
                let is_dir = de.path().is_dir();
                FileEntry { name, is_dir }
            })
            .collect::<Vec<_>>();

        entries.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(entries)
    }

    pub fn read_text(&self, path: &str) -> Result<String, String> {
        let target = self.resolve(path)?;
        fs::read_to_string(&target)
            .map_err(|e| format!("failed to read '{}': {e}", target.display()))
    }

    pub fn write_text(&self, path: &str, content: &str) -> Result<String, String> {
        let target = self.resolve(path)?;
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("failed to create '{}': {e}", parent.display()))?;
        }
        fs::write(&target, content)
            .map_err(|e| format!("failed to write '{}': {e}", target.display()))?;
        Ok(format!(
            "Saved {} bytes to '{}'.",
            content.len(),
            target.display()
        ))
    }

    pub fn search_name(&self, query: &str, limit: usize) -> Vec<String> {
        let mut out = Vec::new();
        walk(&self.home, query, 0, 4, &mut out, limit);
        out
    }

    fn resolve(&self, path: &str) -> Result<PathBuf, String> {
        let base = fs::canonicalize(&self.home)
            .map_err(|e| format!("failed to access home '{}': {e}", self.home.display()))?;

        let candidate = PathBuf::from(path);
        let joined = if candidate.is_absolute() {
            candidate
        } else {
            base.join(candidate)
        };

        let normalized = normalize_path(&joined);
        if !normalized.starts_with(&base) {
            return Err(
                "path denied by sandbox. Next step: use a path under .aether_home.".to_string(),
            );
        }

        Ok(normalized)
    }
}

fn normalize_path(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::ParentDir => {
                out.pop();
            }
            std::path::Component::CurDir => {}
            other => out.push(other.as_os_str()),
        }
    }
    out
}

fn walk(
    root: &Path,
    query: &str,
    depth: usize,
    max_depth: usize,
    out: &mut Vec<String>,
    limit: usize,
) {
    if depth > max_depth || out.len() >= limit {
        return;
    }

    let Ok(rd) = fs::read_dir(root) else {
        return;
    };

    for entry in rd.flatten() {
        if out.len() >= limit {
            break;
        }

        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        if name.to_lowercase().contains(&query.to_lowercase()) {
            out.push(path.display().to_string());
        }

        if path.is_dir() {
            walk(&path, query, depth + 1, max_depth, out, limit);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FileExplorer;

    #[test]
    fn write_then_read_roundtrip() {
        let explorer = FileExplorer::new();
        let path = "target/test-data/aether_roundtrip.txt";
        let payload = "hello legend";
        explorer.write_text(path, payload).expect("write");
        let read_back = explorer.read_text(path).expect("read");
        assert_eq!(read_back, payload);
    }

    #[test]
    fn denies_parent_escape() {
        let explorer = FileExplorer::new();
        let blocked = explorer.read_text("../../etc/passwd");
        assert!(blocked.is_err());
    }
}
