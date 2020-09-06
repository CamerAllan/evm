use crate::evm;
use anyhow::Result;
use std::path;

pub struct EvmConfiguration {
    pub profile_location: path::PathBuf,
    pub config_location: path::PathBuf,
    pub archive_location_relative: path::PathBuf,
    pub active_location_relative: path::PathBuf,
}

impl EvmConfiguration {
    pub fn init(self) -> Result<()> {
        let res = evm::initialise(&self);
        println!("Successfully initialised evm. Please open a new session or source {:?} to start using.", self.profile_location);

        res
    }

    pub fn swap(self, name: &str, version: &str) -> Result<()> {
        let res = evm::swap_to_version(&self, name, version);
        println!("Activated {} {}", name, version);

        res
    }

    pub fn list(self, name: &str) -> Result<()> {
        let active = evm::get_active_version(&self, name)?;
        let versions = evm::list_versions(&self, name)?;

        for entry in versions.iter() {
            let file_name = entry.file_name();
            if active.eq(&file_name.unwrap().to_string_lossy()) {
                print!("->");
            } else {
                print!("  ");
            }
            println!(" {}", entry.file_name().unwrap().to_string_lossy());
        }

        Ok(())
    }

    pub fn active(self, name: &str) -> Result<()> {
        println!("{}", evm::get_active_version(&self, name)?);
        Ok(())
    }

    pub fn add(self, name: &str, version: &str, path: &path::PathBuf) -> Result<()> {
        let res = evm::add_bin_version(&self, name, version, path);
        println!("Added {} {}", name, version);

        res
    }

    pub fn remove(self, name: &str, version: &Option<String>) -> Result<()> {
        let res = evm::remove_bin_version(&self, name, version);
        match version {
            Some(version) => println!("Removed {} {}", name, version),
            None => println!("Removed {}", name),
        }

        res
    }
}

// Helpers
impl EvmConfiguration {
    pub fn active_dir(&self) -> path::PathBuf {
        self.config_location
            .join(self.active_location_relative.clone())
    }
    pub fn active_bin(&self, name: &str) -> path::PathBuf {
        self.active_dir().join(&name)
    }
    pub fn archive_dir(&self) -> path::PathBuf {
        self.config_location
            .join(self.archive_location_relative.clone())
    }
    pub fn archive_bin(&self, name: &str) -> path::PathBuf {
        self.archive_dir().join(&name)
    }
    pub fn archive_bin_ver_dir(&self, name: &str, version: &str) -> path::PathBuf {
        self.archive_dir().join(&name).join(&version)
    }
    pub fn archive_bin_ver(&self, name: &str, version: &str) -> path::PathBuf {
        self.archive_bin_ver_dir(&name, &version).join(&name)
    }
}
