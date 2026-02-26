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
        Self {
            home: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        }
    }

    pub fn list(&self, path: Option<&str>) -> Result<Vec<FileEntry>, String> {
        let target = self.resolve(path.unwrap_or("."));
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
        let target = self.resolve(path);
        fs::read_to_string(&target)
            .map_err(|e| format!("failed to read '{}': {e}", target.display()))
    }

    pub fn write_text(&self, path: &str, content: &str) -> Result<String, String> {
        let target = self.resolve(path);
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

    fn resolve(&self, path: &str) -> PathBuf {
        let candidate = PathBuf::from(path);
        if candidate.is_absolute() {
            candidate
        } else {
            self.home.join(candidate)
        }
    }
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
}
