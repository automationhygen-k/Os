use crate::os::security::SandboxPolicy;

#[derive(Clone, Debug)]
pub enum Platform {
    Native,
    MacOS,
    Windows,
    Android,
}

#[derive(Clone)]
pub struct AppManifest {
    pub id: String,
    pub name: String,
    pub platform: Platform,
    pub entry: String,
    pub args: Vec<String>,
    pub sandbox: SandboxPolicy,
}

#[derive(Default, Clone)]
pub struct AppRegistry {
    apps: Vec<AppManifest>,
}

impl AppRegistry {
    pub fn register(&mut self, app: AppManifest) {
        self.apps.push(app);
    }

    pub fn list(&self) -> &[AppManifest] {
        &self.apps
    }

    pub fn find_by_name(&self, name: &str) -> Option<AppManifest> {
        self.apps
            .iter()
            .find(|app| app.name.eq_ignore_ascii_case(name) || app.id.eq_ignore_ascii_case(name))
            .cloned()
    }
}
