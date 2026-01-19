use serde::Deserialize;
use std::{fmt, fs, path::Path};

#[derive(Debug, Deserialize)]
pub struct Capsule {
    pub name: String,
    pub kernel: String,
    pub initrd: String,
    pub cmdline: Option<String>,
    pub path: String,
}

impl fmt::Display for Capsule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Capsule {
    pub fn scan(root: &str) -> Result<Vec<Self>, ()> {
        let mut out = Vec::new();
        let entries = fs::read_dir(root).map_err(|_| ())?;

        for e in entries.flatten() {
            let path = e.path();
            if !path.is_dir() {
                continue;
            }

            let manifest = path.join("manifest.toml");
            if !manifest.exists() {
                continue;
            }

            let data = fs::read_to_string(&manifest).map_err(|_| ())?;
            let mut cap: Capsule = toml::from_str(&data).map_err(|_| ())?;
            cap.path = path.to_string_lossy().to_string();

            out.push(cap);
        }

        Ok(out)
    }
}
