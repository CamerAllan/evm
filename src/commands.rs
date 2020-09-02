use std::fs::create_dir_all;
use std::path;

pub struct BinSwap {
    pub config_location: path::PathBuf,
    pub inactive_location_relative: path::PathBuf,
    pub active_location_relative: path::PathBuf,
}

// API
impl BinSwap {
    pub fn init(self) -> std::io::Result<()> {
        create_dir_all(self.active_location())?;
        create_dir_all(self.inactive_location())?;
        Ok(())
    }

    pub fn add(self, name: &str, version: &str, path: &path::PathBuf) -> std::io::Result<()> {
        // let create_dir_all();

        Ok(())
    }

    pub fn swap(self, name: &str, version: &str) -> std::io::Result<()> {
        Ok(())
    }

    pub fn remove(self, name: &str, version: &str) -> std::io::Result<()> {
        // let create_dir_all();

        Ok(())
    }
}

// Helpers
impl BinSwap {
    fn active_location(&self) -> path::PathBuf {
        self.config_location
            .join(self.active_location_relative.clone())
    }
    fn inactive_location(&self) -> path::PathBuf {
        self.config_location
            .join(self.inactive_location_relative.clone())
    }
}
