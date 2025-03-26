use std::collections::HashMap;

pub struct PackageManager {
    registry: HashMap<String, Package>,
    cache_dir: PathBuf,
}

impl PackageManager {
    pub fn new() -> Self {
        Self {
            registry: HashMap::new(),
            cache_dir: dirs::cache_dir().unwrap().join("minilang"),
        }
    }

    pub fn install(&mut self, name: &str, version: &str) -> Result<(), String> {
        // Lógica de instalação de pacotes
        Ok(())
    }
}